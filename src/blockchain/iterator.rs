use blockchain::block::Block;
use db::db::{Tree, block_db};
use hex::encode;

#[derive(Clone)]
pub struct Iterator {
    pub current_hash: Vec<u8>,
    pub block_db: Tree
}

impl Iterator {
    pub fn next(mut self) -> (Iterator, Block) {
        println!("iterator block: {:?}", encode(&self.current_hash));
        let _db = block_db();
        let _encode_block = _db.get(&self.current_hash);
        let _block:Block = Block::ds(_encode_block.unwrap().unwrap());
        self.current_hash =  _block.clone().prev_block_hash;
        return (self.clone(), _block);
    }
}
