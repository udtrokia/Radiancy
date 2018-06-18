/*****
 *
 ** /radiancy/src/blockchain/blockchain.rs
 * 
 */

use blockchain::block::{ Block, new_genesis_block, new_block };
use blockchain::iterator::{ Iterator as BlockchainIterator};
use db::db::{Tree, db};
use sled::{ Iter };
use tx::tx::{Transaction, TXOutput, new_coinbase_tx};
use std::collections::HashMap;
use hex::encode;

#[derive(Clone)]
pub struct Blockchain {
    tip: Vec<u8>,
    db: Tree
}

impl Blockchain {
    pub fn iterator(self) -> BlockchainIterator {
        let bci = BlockchainIterator {
            current_hash: self.clone().tip,
            db: self.db
        };
        return bci;
    }
    pub fn find_unspent_transactions(self, address: String) -> Vec<Transaction>{
        let mut unspent_txs: Vec<Transaction> = vec![];
        let mut spent_txos: HashMap<String, Vec<i32>> = HashMap::new();
        let mut bci: BlockchainIterator = self.iterator();        

        loop {
            let (_new_bci, _block) = bci.clone().next();
            bci = _new_bci;
            for tx in _block.transactions {
                let tx_id = encode(tx.clone().id);

                'outputs: for (out_idx, out) in tx.clone().vout.iter().enumerate() {
                    if spent_txos.get(&tx_id).is_some() && spent_txos[&tx_id] != vec![] {
                        println!("------------- iterator self mined out ----------------");
                        for spent_out in &spent_txos[&tx_id] {
                            if spent_out == &(out_idx as i32) {
                                continue 'outputs;
                            }
                        }
                    }
                    
                    if out.to_owned().can_be_unlocked_with(address.to_owned()) {
                        unspent_txs.append(&mut vec![tx.clone()]);
                    }
                }

                if tx.clone().is_coinbase() == false {
                    for _vin in tx.clone().vin {
                        if _vin.clone().can_unlock_output_with(address.to_owned()) {
                            println!("------------- self mined out and check reward  --------------");
                            let in_txid = String::from_utf8(_vin.txid).unwrap();
                            let mut _trans: Vec<i32> = spent_txos.get(&in_txid).unwrap().to_owned();
                            _trans.append(&mut vec![_vin.vout_idx]);
                            spent_txos.remove(&in_txid);
                            spent_txos.insert(in_txid, _trans.to_vec());
                        }
                    }
                }
            }
            if _block.prev_block_hash.len() == 0 { break; }
        }
        return unspent_txs;
    }

    pub fn find_utxo(self, address: String) -> Vec<TXOutput> {
        let mut utxos: Vec<TXOutput> = vec![];
        let unspent_transactions: Vec<Transaction>
            = self.find_unspent_transactions(address.to_owned());

        for tx in unspent_transactions {
            for out in tx.vout {
                if out.to_owned().can_be_unlocked_with(address.to_owned()){
                    utxos.append(&mut vec![out]);
                }
            }
        }
        return utxos;
    }
    
    pub fn find_spendable_outputs(self, address: String, _amount: i32) -> (i32, HashMap<String, Vec<i32>>) {
        let mut _unspent_outputs:HashMap<String, Vec<i32>> = HashMap::new();
        let _unspent_txs = self.find_unspent_transactions(address.to_owned());
        let mut _accumulated = 0;
        for tx in _unspent_txs {
            let in_txid = encode(tx.clone().id);
            'work: for (_out_idx, out) in tx.clone().vout.iter().enumerate() {
                if out.clone().can_be_unlocked_with(address.to_owned())
                    && _accumulated < _amount {
                        _accumulated = _accumulated + out.value;
                        if _unspent_outputs.get(&in_txid).is_none() {
                            break 'work;
                        }
                        let mut _trans = _unspent_outputs.get(&in_txid).unwrap().to_owned();
                        _trans.append(&mut vec![(_out_idx as i32)]);
                        _unspent_outputs.remove(&in_txid);
                        _unspent_outputs.insert(in_txid.to_owned(), _trans);

                        if _accumulated >= _amount {
                            break 'work;
                        }
                    }
            }
        }
        return (_accumulated, _unspent_outputs);
    }

    pub fn mine_block(self, transactions: Vec<Transaction>) {
        let _db = db();
        let last_hash: Vec<u8> = self.clone()
            .db.get(&"last".to_string().into_bytes()).unwrap().unwrap();

        let new_block: Block = new_block(transactions, last_hash);

        let _set_hash = self.db.set(new_block.clone().hash, new_block.clone().serialize());
        if _set_hash.is_ok() == false { panic!(_set_hash.unwrap()) };
        let _set_last = self.db.set("last".to_string().into_bytes(), new_block.clone().hash);
        if _set_last.is_ok() == false { panic!(_set_last.unwrap()) };
    }
}


pub fn new_blockchain(address:String) -> Blockchain {
    let _db = db();
    let _dbc = db().clone();
    let tip: Vec<u8>;
    let iter:Iter = _dbc.scan(b"no previous block...");
    
    let height = iter.count();
    let genesis_coinbase_data = "I'm genesis coinbase data".to_string();
    if height == 0 {
        let cbtx = new_coinbase_tx(address, genesis_coinbase_data);
        let genesis:Block = new_genesis_block(cbtx);
        
        let _store_hash = _db.set(genesis.clone().hash, genesis.clone().serialize());
        if _store_hash.is_ok() == false { panic!(_store_hash.unwrap())}
        let _store_last = _db.set("last".to_string().into_bytes(), genesis.clone().hash);
        if _store_last.is_ok() == false { panic!(_store_last.unwrap())}
        tip = genesis.hash;
    } else {
        tip = _db.get(&"last".to_string().into_bytes()).unwrap().unwrap().to_vec();
    }
    let _new_blockchain = Blockchain {
        db: _db,
        tip: tip
    };
    return _new_blockchain;
}

