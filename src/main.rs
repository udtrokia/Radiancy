/*****
  *
  ** udtrokia<udtrokia@gmail.com>
  * 
 */
//extern crate colored; 
extern crate radiancy; 
//
//use colored::*; 
use radiancy::blockchain::blockchain::{Blockchain, new_blockchain};
use radiancy::cli::cli::CLI;

fn main() {
    let _blockchain: Blockchain = new_blockchain();
    let _cli:CLI = CLI{
        blockchain: _blockchain,
    };
    _cli.run();
//    blockchain = blockchain.add_block(String::from("This is Major Tom to Ground Control."));
//    blockchain = blockchain.add_block(String::from("I am stepping through the door."));
}
