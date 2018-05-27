/*****
    *
 ** /radiancy/src/blockchain/block.rs
 * 
*/
extern crate crypto;

use std::time::{SystemTime, UNIX_EPOCH};
use blockchain::block::crypto::sha2::Sha256;
use blockchain::block::crypto::digest::Digest;

pub struct Block {
    pub timestamp: String,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String
}

impl Block {
    fn set_hash(self) -> Block {
        let mut hasher = Sha256::new();
        let header: String = String::new()
            + &self.timestamp + &self.data + &self.prev_block_hash;
        hasher.input_str(&header);
        
        return Block {
            hash: String::from(hasher.result_str().as_str()),
            ..self
        }
    }
}

pub fn new_block(data: String, prev_block_hash: String) -> Block {
    let block = Block {
        timestamp: ts(),
        data: data,
        prev_block_hash: prev_block_hash,
        hash: String::new()
    };
    block.set_hash()
}

pub fn new_genesis_block() -> Block {
    return new_block(String::from("I am Genesiis Block"), String::new());
}

pub fn ts() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("HaHa, Time went backwards!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms.to_string()
}
