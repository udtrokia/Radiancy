/*****
    *
  ** /radiancy/src/pow/pow.rs
  *
 */

use num_bigint::{BigInt, Sign};
use sha2::{Sha256, Digest};
use blockchain::block::Block;
use std::cmp::Ordering;
use std::ops::Shl;


#[derive(Clone)]
pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt
}

impl ProofOfWork {
    pub fn prepare_data(self, nonce:i32) -> Vec<u8> {
        let mut data_camp:Vec<u8> = self.block.timestamp.clone();

        data_camp.append(&mut self.block.prev_block_hash.clone());
        data_camp.append(&mut self.block.hash_transactions().clone());        
        data_camp.append(&mut nonce.to_string().into_bytes());

        return data_camp.to_vec();
    }

    pub fn run(self) -> (Vec<u8>, Vec<u8>) {
        let mut hash_int:BigInt = BigInt::from(1);
        let mut hasher:Sha256;
        let mut nonce:i32 = 0;

        println!("\n`num` crate is really slow, please have patient :\\");
        println!("I've setted the difficulty lowest. in my mbp, it's about 10mins...");
        println!("But, don't worry, I'll replace the PoW module soon.\n");
        //println!("Mining the block containing: {:?}",
        //String::from_utf8(self.clone().block.data).unwrap());        
        while nonce < i32::max_value() {
            let data = self.clone().prepare_data(nonce);
            hasher = Sha256::default();
            hasher.input(&data);
            hash_int = BigInt::from_bytes_be(Sign::Plus, &hasher.clone().result());
            if hash_int.cmp(&self.target) == Ordering::Less {
                println!("Mining out block: {:x}", &hasher.result());            
                break;
            } else {
                nonce += 1;
            }
        } 
        return (nonce.to_string().into_bytes(), hash_int.to_bytes_be().1);
    }

    pub fn validate(self) -> bool {
        let _hash_int: BigInt;
        let mut _data = self.clone().prepare_data(
            i32::from_str_radix(
                &String::from_utf8(
                    self.clone().block.nonce
                ).unwrap(),
                16).unwrap()
        );
        let mut _hasher = Sha256::default();
        _hasher.input(&_data);
        _hash_int = BigInt::from_bytes_be(Sign::Plus, &_hasher.result());
        
        let is_vaild:bool = _hash_int.cmp(&self.target) == Ordering::Less;
        return is_vaild;
    }
}

pub fn new_proof_of_work(b:Block, target_bits:i32) -> ProofOfWork {
    let pre_target: BigInt = BigInt::from(1);
    let _target = pre_target.clone().shl(256 as usize - target_bits as usize );
    let pow: ProofOfWork = ProofOfWork { block: b, target: _target };
    return pow;
}
