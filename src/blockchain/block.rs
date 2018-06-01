/*****
 *
 ** /radiancy/src/blockchain/block.rs
 * 
 */
// extern crate bytes;
extern crate crypto;

// use self::bytes::Bytes;
use self::crypto::sha2::Sha256;
use self::crypto::digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    pub timestamp: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>
}

impl Block {
    fn set_hash(self) -> Block {
        let mut hasher = Sha256::new();
        let _data = &self.data.clone();
        let header = String::new()
            + &String::from_utf8(self.timestamp.clone()).unwrap()
            + &String::from_utf8(_data.to_vec()).unwrap() + &String::from_utf8(self.prev_block_hash.clone()).unwrap();
        // header = header.append();
        hasher.input_str(&header);
        
        return Block {
            hash: hasher.result_str().into_bytes(),
            ..self
        }
    }
}

pub fn new_block(data: String, prev_block_hash: Vec<u8>) -> Block {
    let block = Block {
        timestamp: ts(),
        data: data.into_bytes(),
        prev_block_hash: prev_block_hash,
        hash: Vec::new()
    };
    block.set_hash()
}

pub fn new_genesis_block() -> Block {
    return new_block(String::from("I am Genesiis Block"), Vec::new());
}

pub fn ts() -> Vec<u8> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("HaHa, Time went backwards!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms.to_string().into_bytes();
}
