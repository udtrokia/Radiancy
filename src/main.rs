/*****
 *
 ** udtrokia<udtrokia@gmail.com>
 * 
*/
extern crate colored; 
extern crate radiancy; 

use colored::*; 
use radiancy::blockchain::blockchain::{ Blockchain, new_blockchain };
use radiancy::pow::pow::new_proof_of_work;
use radiancy::pow::pow::{BigInt, Sign};

fn main() {
    let mut blockchain: Blockchain = new_blockchain();
    blockchain = blockchain.add_block(String::from("This is Major Tom to Ground Control."));
    blockchain = blockchain.add_block(String::from("I am stepping through the door."));
    for _block in blockchain.blocks {
        let pow = new_proof_of_work(_block.clone(), 0);
        println!("\nPrev. hash: {:x}", BigInt::from_bytes_be(Sign::Plus, &_block.prev_block_hash));
	println!("Data: {}", String::from_utf8(_block.data.to_vec()).unwrap().magenta());
        println!("Hash: {:x}", BigInt::from_bytes_be(Sign::Plus, &_block.hash));
	println!("Time: {}", String::from_utf8(_block.timestamp).unwrap().underline().blue());
	println!("PoW:  {}", pow.validate().to_string().green());
    };
}
