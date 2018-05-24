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

pub struct Blockchain {
    blocks: [Block]
}

fn ts() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms.to_string()
}

fn set_hash(b: Block) -> Block {
    let mut hasher = Sha256::new();
    let header: String = b.timestamp + &b.data + &b.prev_block_hash;
    hasher.input_str(&header);
    
    let block = Block {
        hash: hasher.result_str(),
        timestamp: b.timestamp,
        data: b.data,
        prev_block_hash: b.prev_block_hash
    };
    return block;
}

fn new_block(data: String, prev_block_hash: String) -> Block {
    let block = Block {
        timestamp: ts(),
        data: data,
        prev_block_hash: prev_block_hash,
        hash: "".to_string()
    };
//    println!("{}{}{}{}", block.timestamp, block.data, block.prev_block_hash, block.hash);
//    let hash = set_hash(block);
//    println!("{}", hash);
    return block;
}

fn main() {
    new_block("".to_string(), "".to_string());
}
