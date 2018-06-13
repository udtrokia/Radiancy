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

struct TXO_Pair {
    String: Vec<i32>,
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
        // unspentTXs
        // spentTXOs
        let spent_TXOs: HashMap<String, Vec<i32>> = HashMap::new();
        let bci: BlockchainIterator = self.iterator();
        loop {
            let (_new_bci, _block) = bci.clone().next();
            for tx in _block.transactions {
                let tx_id = String::from_utf8(tx.id).unwrap();
                // this place, iter().enumerate().ls
                'outputs: for (out_idx, out) in tx.vout.iter().enumerate() {
                    if spent_TXOs.clone()[&tx_id] != vec![] {
                        for spent_out in &spent_TXOs[&tx_id] {
                            if spent_out == &(out_idx as i32) {
                                continue 'outputs;
                            }
                        }
                    }
                }
            }
        }
    }    
}

//sled::DbResult
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
