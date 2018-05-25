extern crate bytes;
extern crate crypto;

use std::time::{SystemTime, UNIX_EPOCH};
use crypto::digest::Digest;
use crypto::sha2::Sha256;

//use std::io::{self, Read};
//use bytes::{Bytes};

pub struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String
}

impl Block {
    fn set_hash(self) -> Block {
        let mut hasher = Sha256::new();
        let header: String = String::new()  + &self.timestamp + &self.data + &self.prev_block_hash;
        hasher.input_str(&header);

        return Block {
            hash: String::from(hasher.result_str().as_str()),
            ..self
        }
    }
}

pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    fn add_block(&mut self, data: String) {
        let ref prev_block: Block = self.blocks[self.blocks.len() - 1]; // immutable
        let _new_block: Block = new_block(data, String::from(prev_block.hash.as_str()));
        self.blocks.push(_new_block)
    }
}

fn new_block(data: String, prev_block_hash: String) -> Block {
    let block = Block {
        timestamp: ts(),
        data: data,
        prev_block_hash: prev_block_hash,
        hash: String::new()
    };
    block.set_hash()
}

fn new_genesis_block() -> Block {
    return new_block(String::from("Genesiis Block"), String::new());
}

fn new_blockchain() -> Blockchain {
    return Blockchain {blocks: vec![new_genesis_block()]}
}

fn ts() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms.to_string()
}

fn main() {
    new_block("".to_string(), "".to_string());
}
