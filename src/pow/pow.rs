/*****
    *
  ** /radiancy/src/pow/pow.rs
  *
 */
extern crate num_bigint;

use self::num_bigint::{BigInt, Sign};
use std::ops::Shl;
use blockchain::block::Sha256;
use blockchain::block::Digest;
use blockchain::block::Block;

#[derive(Clone)]
pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt
}

impl ProofOfWork {
    pub fn prepare_data(self, nonce:i32) -> String {
        let data_camp:String = String::new()
            + &String::from_utf8(self.block.timestamp.clone()).unwrap()
            + &String::from_utf8(self.block.data.clone()).unwrap()
            + &String::from_utf8(self.block.prev_block_hash.clone()).unwrap() 
            + &String::from(nonce.to_string());
        return data_camp;
    }

    pub fn run(self) -> (i32, Vec<u8>) {
        let mut hash_int:BigInt = BigInt::from(1);
        let mut hasher = Sha256::new();
        let mut nonce:i32 = 0;
        let data = self.clone().prepare_data(nonce);

        while nonce < i32::max_value() {
            hasher.input_str(&data);
            let mut _hash = hasher.result_str();
            hash_int = BigInt::from_bytes_be(Sign::Plus, &_hash.into_bytes());
            
            if hash_int.eq(&self.target) == false {
                break;
            } else {
                nonce += 1;
            }
            println!("\n");
        } 

        return (nonce, hash_int.to_bytes_be().1);
    }

    pub fn validate(self) -> bool {
        let _hash_int: BigInt;
        let _data = &self.clone().prepare_data(self.clone().block.nonce);
        let mut _hasher = Sha256::new();
        _hasher.input_str(&_data);
        let _hash = _hasher.result_str();
        _hash_int = BigInt::from_bytes_be(Sign::Plus, &_hash.into_bytes());
        
        let is_vaild:bool = _hash_int.eq(&self.target);
        return is_vaild;
    }
}

pub fn new_proof_of_work(b:Block, target_bits:i32) -> ProofOfWork {
    let pre_target: BigInt = BigInt::from(1);
    let _target = pre_target.clone().shl(256 as usize - target_bits as usize );
    let pow: ProofOfWork = ProofOfWork { block: b, target: _target };
    return pow;
}
