use pow::pow::{ProofOfWork, Block, BigInt};
use std::ops::Shl;

pub fn new_proof_of_work(b:Block, target_bits:i32) -> ProofOfWork {
    let pre_target: BigInt = BigInt::from(1);
    let _target = pre_target.clone().shl(256 as usize - target_bits as usize );
    let pow: ProofOfWork = ProofOfWork { block: b, target: _target };
    return pow;
}
