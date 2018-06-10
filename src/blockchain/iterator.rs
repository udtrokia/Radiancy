use blockchain::block::{Block, deserialize_block};
use db::db::{Tree, db};
    
pub struct Iterator {
    pub current_hash: Vec<u8>,
    pub db: Tree
}

impl Iterator {
    pub fn next(self) -> Block {
        let _db = db();
        let _encode_block = _db.get(&self.current_hash);
        let _block:Block = deserialize_block(_encode_block.unwrap().unwrap());
        
        return _block
    }
}
