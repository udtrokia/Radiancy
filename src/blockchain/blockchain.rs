/*****
    *
 ** /radiancy/src/blockchain/blockchain.rs
 * 
*/
use blockchain::block::{
    Block, new_block, new_genesis_block
};
use blockchain::iterator::{ Iterator as BlockchainIterator};
use db::db::{Tree, db};
use sled::{ Iter };
use tx::tx::new_coinbase_tx;

#[derive(Clone)]
pub struct Blockchain {
    tip: Vec<u8>,
    db: Tree
}

impl Blockchain {
    //pub fn add_block(self, data:String) {
    //    let _db = db();
    //    let last_hash: Vec<u8> = self.clone()
    //        .db.get(&"last".to_string().into_bytes()).unwrap().unwrap();
    //
    //    let new_block: Block = new_block(data, last_hash);
    //
    //    let _set_hash = self.db.set(new_block.clone().hash, new_block.clone().serialize());
    //    if _set_hash.is_ok() == false { panic!(_set_hash.unwrap()) };
    //    let _set_last = self.db.set("last".to_string().into_bytes(), new_block.clone().hash);
    //    if _set_last.is_ok() == false { panic!(_set_last.unwrap()) };
    //
    //}
    pub fn iterator(self) -> BlockchainIterator {
        let bci = BlockchainIterator {
            current_hash: self.clone().tip,
            db: self.db
        };
        return bci;
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
