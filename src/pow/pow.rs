/*****
    *
  ** /radiancy/src/pow/pow.rs
  *
 */
extern crate num_bigint;
extern crate sha2;
extern crate hex;

pub use self::num_bigint::{BigInt, Sign};
use self::sha2::{Sha256, Digest};
//use blockchain::block::byteorder::{LittleEndian, WriteBytesExt};
use blockchain::block::Block;
use std::cmp::Ordering;
//use std::mem;
use std::ops::Shl;


#[derive(Clone)]
pub struct ProofOfWork {
    pub block: Block,
    pub target: BigInt
}

impl ProofOfWork {
    pub fn prepare_data(self, nonce:i32) -> Vec<u8> {
        //let mut _nonce = [0u8; mem::size_of::<i32>()];
        //_nonce.as_mut().write_i32::<LittleEndian>(nonce).expect("Unable to write");
        let mut data_camp:Vec<u8> = self.block.timestamp.clone();
        data_camp.append(&mut self.block.data.clone());
        data_camp.append(&mut self.block.prev_block_hash.clone());
        data_camp.append(&mut nonce.to_string().into_bytes());

        return data_camp.to_vec();
    }

    pub fn run(self) -> (i32, Vec<u8>) {
        let mut hash_int:BigInt = BigInt::from(1);
        let mut hasher:Sha256;
        let mut nonce:i32 = 0;

        println!("\nMinning start... `num` crate is really slow, please have patient :\\");
        while nonce < i32::max_value() {
            let data = self.clone().prepare_data(nonce);
            hasher = Sha256::default();
            hasher.input(&data);
            hash_int = BigInt::from_bytes_be(Sign::Plus, &hasher.clone().result());
            if hash_int.cmp(&self.target) == Ordering::Less {
                println!("Mining out block: {:x}", &hasher.result());
                println!("Data: {:?}", String::from_utf8(self.block.data).unwrap());
                break;
            } else {
                nonce += 1;
            }
        } 
        return (nonce, hash_int.to_bytes_be().1);
    }

    pub fn validate(self) -> bool {
        let _hash_int: BigInt;
        let mut _data = self.clone().prepare_data(self.clone().block.nonce);
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
