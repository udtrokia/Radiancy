extern crate bytes;
extern crate colored;
extern crate crypto;

use colored::*;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

struct Block {
    data: String,
    hash: String,
    prev_block_hash: String,
    timestamp: String
}

impl Block {
    fn set_hash(self) -> Block {
        let mut hasher = Sha256::new();
        let header: String = String::new()
            + &self.timestamp + &self.data + &self.prev_block_hash;
        // Mix header
        hasher.input_str(&header);
        // Set hash
        return Block {
            hash: String::from(hasher.result_str().as_str()),
            ..self
        }
    }
}

fn new_block(data: String, prev_block_hash: String) -> Block {
    let block = Block {
        data: data,
        hash: String::new(),        
        timestamp: ts(),
        prev_block_hash: prev_block_hash
    };
    // Cover the reserve hash.
    block.set_hash()
}

fn new_genesis_block() -> Block {
    return new_block(String::from("Genesiis Block"), String::new());
}

struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    fn get_prev_hash(&self) -> String {
        let prev_block: &Block = &self.blocks[self.blocks.len() -1];
        let prev_hash: String = (&prev_block.hash).to_string();
        // Get pre-hash from the last block.
        return prev_hash;
    }
    fn add_block(mut self, data: String) -> Blockchain {
        let _prev_hash: String = self.get_prev_hash();
        let _new_block: Block = new_block(data, String::from(_prev_hash));
        self.blocks.push(_new_block);
        // Push new block into blockchain.
        return Blockchain {
            blocks: self.blocks
        };
    }
}

fn new_blockchain() -> Blockchain {
    let mut _new_blockchain = Blockchain {
        blocks: vec![new_genesis_block()]
    };
    // Generate a new blockchain.
    return _new_blockchain;
}

fn ts() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    // Just fetch the UNIX time.
    return in_ms.to_string()
}

fn main() {
    let mut blockchain: Blockchain = new_blockchain();        
    // Generate a new blockchain in main function.
    blockchain = blockchain.add_block(String::from("Hi, Pluto. I'm Basquiat."));
    blockchain = blockchain.add_block(String::from("Can you be my friend?"));
    // Print block infos.
    for _block in blockchain.blocks {
        println!("\nPrev. hash: {}", _block.prev_block_hash.yellow());
		    println!("Data: {}", _block.data.magenta());
		    println!("Hash: {}\n", _block.hash.bold().underline().cyan());
    };
}
