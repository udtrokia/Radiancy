/*****
    *
  ** /radiancy/src/blockchain/block.rs
 * 
 */

use std::time::{SystemTime, UNIX_EPOCH};
use pow::pow::new_proof_of_work;
use bincode::{serialize, deserialize};
use tx::tx::{Transaction};
use sha2::{Sha256, Digest};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Block {
    pub timestamp: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
    pub nonce: Vec<u8>
}

impl Block {
    pub fn serialize (self) -> Vec<u8> {
        let encode:Vec<u8> = serialize(&self).unwrap();
        return encode;
    }
    pub fn hash_transactions(self) -> Vec<u8> {
        let mut tx_hashes: Vec<u8> = vec![];
        
        for mut tx in self.transactions {
            tx_hashes.append(&mut tx.id);
        }

        let mut hasher: Sha256 = Sha256::default();
        hasher.input(&tx_hashes);
        return hasher.result().to_vec();
    }
}

pub fn deserialize_block(data: Vec<u8>) -> Block {
    let decode:Block = deserialize(&data).unwrap();
    return decode;
}

pub fn new_block(transactions: Vec<Transaction>, prev_block_hash: Vec<u8>) -> Block {
    let block:Block = Block {
        timestamp: ts(),
        transactions: transactions,
        prev_block_hash: prev_block_hash,
        hash: Vec::new(),
        nonce: Vec::new()
    };
    
    let _pow = new_proof_of_work(block, 0);
    let (_nonce, _hash) = _pow.clone().run();
    let _block:Block = Block {
        hash: _hash,
        nonce: _nonce,
        .._pow.block
    };
    return _block;
}

pub fn new_genesis_block( coinbase: Transaction ) -> Block {
    return new_block(vec![coinbase], Vec::new());
}

pub fn ts() -> Vec<u8> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)
        .expect("HaHa, Time went backwards!");
    let in_ms = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;

    return in_ms.to_string().into_bytes();
}
