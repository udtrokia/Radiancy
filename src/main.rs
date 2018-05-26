extern crate bytes;
extern crate crypto;

use std::time::{SystemTime, UNIX_EPOCH};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
//use std::io::{self, Read};
//use bytes::{Bytes};

struct Block {
    timestamp: String,
    data: String,
    prev_block_hash: String,
    hash: String
}

impl Block {
    fn set_hash(self) -> Block {
        let mut hasher = Sha256::new();
        let header: String = String::new()  +
            &self.timestamp + &self.data + &self.prev_block_hash;
        hasher.input_str(&header);

        return Block {
            hash: String::from(hasher.result_str().as_str()),
            ..self
        }
    }
}

#[warn(unused_mut)]    
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

struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    #[warn(dead_code)]
    fn get_prev_hash(&self) -> String {
        let prev_block: &Block = &self.blocks[self.blocks.len() -1];
        let prev_hash: String = (&prev_block.hash).to_string();
        return prev_hash;
    }
    fn add_block(mut self, data: String) -> Blockchain {
        // to copy self with value to new RAM address
        // use reference to get the value of _self but not move the ownership.
        // let prev_block: &Block = &self.blocks[&self.blocks.len() -1];
        // let prev_hash: String = (&prev_block.hash).to_string();
        // &self.blocks[&self.blocks.len() - 1]; // immutable

        let _prev_hash: String = self.get_prev_hash();
        let _new_block: Block = new_block(data, String::from(_prev_hash));
        // let _new_block: Block = new_block(data, _prev_hash.to_string());
        println!("  new block data: {:?}", _new_block.data);
        // let _blocks = (self.blocks).clone();

        self.blocks.push(_new_block);
        return Blockchain {
            blocks: self.blocks
        };
        // self.blocks.push(_new_block)
        // _blocks.push(_new_block)
        // return Blockchain {}
        // let _block: Block = new_genesis_block();
        // println!("  new genesisblock hash:\n {:?}", _block.hash);
        // This is a big problem!!! I need no mut but this.
        // self.blocks.push(_new_block)
    }
}

fn new_blockchain() -> Blockchain {
    let mut _new_blockchain = Blockchain {
        // blocks: vec![new_genesis_block()]
        blocks: vec![new_genesis_block()]
    };
    return _new_blockchain;
}


fn ts() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    return in_ms.to_string()
}

//#[derive(Debug)]
fn main() {
    // let blockchain: &Blockchain = &mut new_blockchain();
    // let a:Blockchain = blockchain.add_block(String::from("hello, world"));
    let mut blockchain: Blockchain = new_blockchain();        
    // {
    //     let mut _add_block = |_data: String| blockchain.add_block(_data);
    //     _add_block(String::from("hello, world"));
    //     _add_block(String::from("hello, world"));        
    // }
    blockchain = blockchain.add_block(String::from("hello, world"));
    blockchain = blockchain.add_block(String::from("hello, world"));    
    // blockchain.add_block(String::from("hello, world"));    
    // blockchain.add_block(String::from("hello, world"));
    // let _block: Block = new_genesis_block();
    // println!("{:?}", _block.hash)
    // println!("{:?}", a.blocks[0].hash) 
}
