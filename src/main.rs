/*****
    *
 ** udtrokia<udtrokia@gmail.com>
 * 
*/
extern crate colored; 
extern crate radiancy; 

use colored::*; 
use radiancy::blockchain::blockchain::{
    Blockchain, new_blockchain
};

fn main() {
    let mut blockchain: Blockchain = new_blockchain();        
    blockchain = blockchain.add_block(String::from("Hi, Pluto. I'm Basquiat."));
    blockchain = blockchain.add_block(String::from("Can you be my friend?"));
    for _block in blockchain.blocks {
        println!("\nPrev. hash: {}", _block.prev_block_hash.yellow());
		    println!("Data: {}", _block.data.magenta());
		    println!("Hash: {}", _block.hash.cyan());
		    println!("Time: {}\n", _block.timestamp.underline().blue());
    };
}
