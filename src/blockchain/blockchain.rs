/*****
    *
 ** /radiancy/src/blockchain/blockchain.rs
 * 
*/
use blockchain::block::{
    Block, new_block, new_genesis_block
};
use db::db::db;
use sled::{ Iter };

pub struct Blockchain { pub blocks: Vec<Block> }
impl Blockchain {
    pub fn get_prev_hash(&self) -> Vec<u8> {
        let prev_block: &Block = &self.blocks[self.blocks.len() -1];
        let prev_hash: Vec<u8> = prev_block.hash.to_vec();
        return prev_hash;
    }
    pub fn add_block(mut self, data: String) -> Blockchain {
        let _prev_hash: Vec<u8> = self.get_prev_hash();
        let _new_block: Block = new_block(data, _prev_hash);
        
        self.blocks.push(_new_block);
        return Blockchain {
            blocks: self.blocks
        };
    }
}

//sled::DbResult
pub fn new_blockchain() -> Blockchain {
    let _db = db();
    let mut iter:Iter = _db.scan(b"no previous block...");
    let height = iter.count();
//    if height == 0 {
//        
//    }
    let mut _new_blockchain = Blockchain {
        blocks: vec![new_genesis_block()]
    };
    return _new_blockchain;
}
