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

#[derive(Clone)]
pub struct Blockchain {
    tip: Vec<u8>,
    db: Tree    
}

impl Blockchain {
    //pub fn get_prev_hash(&self) -> Vec<u8> {
    //    let prev_block: &Block = &self.blocks[self.blocks.len() -1];
    //    let prev_hash: Vec<u8> = prev_block.hash.to_vec();
    //    return prev_hash;
    //}
    //pub fn add_block(mut self, data: String) -> Blockchain {
    //    let _prev_hash: Vec<u8> = self.get_prev_hash();
    //    let _new_block: Block = new_block(data, _prev_hash);
    //    
    //    self.blocks.push(_new_block.clone());
    //    return Blockchain {
    //        blocks: self.blocks,
    //        tip: _new_block.hash
    //    };
    //}
    pub fn add_block(self, data:String) {
        let _db = db();
        let last_hash: Vec<u8> = self.clone()
            .db.get(&"last".to_string().into_bytes()).unwrap().unwrap();

        let new_block: Block = new_block(data, last_hash);

        let _set_hash = self.db.set(new_block.clone().hash, new_block.clone().serialize());
        if _set_hash.is_ok() == false { panic!(_set_hash.unwrap()) };
        let _set_last = self.db.set("last".to_string().into_bytes(), new_block.clone().hash);
        if _set_last.is_ok() == false { panic!(_set_last.unwrap()) };

    }
    pub fn iterator(self) -> BlockchainIterator {
        return BlockchainIterator {
            current_hash: self.clone().tip,
            db: self.db
        };
    }
}

//sled::DbResult
pub fn new_blockchain() -> Blockchain {
    let _db = db();
    let _dbc = db().clone();
    let tip: Vec<u8>;
    let iter:Iter = _dbc.scan(b"no previous block...");
    
    let height = iter.count();
    if height == 0 {
        let genesis:Block = new_genesis_block();
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
