/*****
    *
  ** /radiancy/src/blockchain/block.rs
 * 
 */
//pub extern crate byteorder;

//use self::byteorder::{LittleEndian, WriteBytesExt};
//use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use pow::pow::new_proof_of_work;
    
#[derive(Clone)]
pub struct Block {
    pub timestamp: Vec<u8>,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: i32
}

pub fn new_block(data: String, prev_block_hash: Vec<u8>) -> Block {
    let block:Block = Block {
        timestamp: ts(),
        data: data.into_bytes(),
        prev_block_hash: prev_block_hash,
        hash: Vec::new(),
        nonce: i32::min_value()
    };
    
    let _pow = new_proof_of_work(block, 24);
    let (_nonce, _hash) = _pow.clone().run();
    let _block:Block = Block {
        hash: _hash,
        nonce: _nonce,
        .._pow.block
    };
    return _block;
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

    //let mut bs = [0u8; mem::size_of::<u64>()];
    //bs.as_mut().write_u64::<LittleEndian>(in_ms).expect("Unable to write");

    return in_ms.to_string().into_bytes();
}
