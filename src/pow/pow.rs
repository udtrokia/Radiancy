/*****
    *
  ** /radiancy/src/pow/pow.rs
  *
 */
extern crate num_bigint;
// extern crate bytes;

use self::num_bigint::{BigInt};
// use self::bytes::Bytes;
use std::ops::Shl;

use blockchain::block::Block;

pub struct ProofOfWork {
    block: Block,
    target: BigInt
}

impl ProofOfWork {
    pub fn prepareData(nonce:i32) {

    }
}

pub fn new_proof_of_work(b:Block, target_bits:i32) -> ProofOfWork {
    let pre_target: BigInt = BigInt::from(2);
    let _target = pre_target.shl((256 - target_bits) as usize);

    let pow: ProofOfWork = ProofOfWork { block: b, target: _target };
    return pow;
}

