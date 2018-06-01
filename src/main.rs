/*****
 *
 ** udtrokia<udtrokia@gmail.com>
 * 
 */
extern crate colored; 
extern crate radiancy; 

use colored::*; 
use radiancy::blockchain::blockchain::{ Blockchain, new_blockchain };

fn main() {
    let mut blockchain: Blockchain = new_blockchain();
    blockchain = blockchain.add_block(String::from("This is Major Tom to Ground Control."));
    blockchain = blockchain.add_block(String::from("I am stepping through the door."));
    for _block in blockchain.blocks {
        println!("\nPrev. hash: {}", String::from_utf8(_block.prev_block_hash).unwrap().yellow());
	println!("Data: {}", String::from_utf8(_block.data.to_vec()).unwrap().magenta());
	println!("Hash: {}", String::from_utf8(_block.hash).unwrap().cyan());
	println!("Time: {}\n", String::from_utf8(_block.timestamp).unwrap().underline().blue());
    };
}
