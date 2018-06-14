/*****
    *
 ** /radiancy/src/blockchain/blockchain.rs
 * 
*/
use blockchain::block::{ Block, new_genesis_block };
use blockchain::iterator::{ Iterator as BlockchainIterator};
use db::db::{Tree, db};
use sled::{ Iter };
use tx::tx::{Transaction, new_coinbase_tx};
use std::collections::HashMap;

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
        let bci: BlockchainIterator = self.iterator(); 
        loop {
            let (_new_bci, _block) = bci.clone().next();
            for tx in _block.transactions {
                let tx_id = String::from_utf8(tx.clone().id).unwrap();
                'outputs: for (out_idx, out) in tx.clone().vout.iter().enumerate() {
                    if spent_txos.clone()[&tx_id] != vec![] {
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
                    for _vin in tx.vin {
                        if _vin.clone().can_unlock_output_with(address.to_owned()) {
                            let in_txid = String::from_utf8(_vin.txid).unwrap();
                            let mut _trans: Vec<i32> = spent_txos.get(&in_txid).unwrap().to_owned();
                            _trans.append(&mut vec![_vin.vout]);
                            spent_txos.remove(&in_txid);
                            spent_txos.insert(in_txid, _trans.to_vec());
                        }
                    }
                }
            }
            if _block.prev_block_hash.len() == 0 {
                break;
            }            
        }
        return unspent_txs;
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
