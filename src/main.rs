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
    let _blockchain: Blockchain = new_blockchain("hello,wrold".to_string());
    let _cli:CLI = CLI{
        blockchain: _blockchain,
    };
    _cli.run();
}
