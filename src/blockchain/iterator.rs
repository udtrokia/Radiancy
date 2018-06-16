use blockchain::block::{Block, deserialize_block};
use db::db::{Tree, db};
use hex::encode;

#[derive(Clone)]
pub struct Iterator {
    pub current_hash: Vec<u8>,
    pub db: Tree
}

impl Iterator {
    pub fn next(mut self) -> (Iterator, Block) {
        println!("iterator block: {:?}", encode(&self.current_hash));
        let _db = db();
        let _encode_block = _db.get(&self.current_hash);
        let _block:Block = deserialize_block(_encode_block.unwrap().unwrap());
        self.current_hash =  _block.clone().prev_block_hash;
        return (self.clone(), _block);
    }
}
