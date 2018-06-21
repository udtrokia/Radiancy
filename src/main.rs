/*****
  *
  ** udtrokia<udtrokia@gmail.com>
  * 
 */
//extern crate colored; 
extern crate radiancy;

use radiancy::blockchain::blockchain::{Blockchain};
use radiancy::blockchain::utils::new_blockchain;
use radiancy::cli::cli::CLI;
use radiancy::wallet::utils::{load_account, hash_pubkey, create_account};

fn main() {
    create_account();
    let _pubkey_hash = hash_pubkey(load_account().pub_key);
    let _blockchain: Blockchain = new_blockchain(_pubkey_hash);
    let _cli:CLI = CLI{ blockchain: _blockchain };
    _cli.run();
}
