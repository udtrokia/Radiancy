/*****
  *
  ** udtrokia<udtrokia@gmail.com>
  * 
 */
extern crate radiancy;

use std::fs;
use std::env::home_dir;

use radiancy::blockchain::blockchain::{Blockchain};
use radiancy::blockchain::utils::new_blockchain;
use radiancy::cli::cli::CLI;
use radiancy::wallet::utils::{load_account, hash_pubkey, create_account};

fn main() {
    
    let mut db_path = home_dir().unwrap();
    db_path.push(".radiancy/db");
    let mut wallet_path = home_dir().unwrap();
    wallet_path.push(".radiancy/wallet");
    
    if fs::metadata(db_path.to_owned()).is_err() {
        assert_eq!(fs::create_dir_all(db_path).is_ok(), true);
    }
    if fs::metadata(wallet_path.to_owned()).is_err() {
        assert_eq!(fs::create_dir_all(wallet_path).is_ok(), true);
    }
    create_account();
    
    let _pubkey_hash = hash_pubkey(load_account().pub_key);
    let _blockchain: Blockchain = new_blockchain(_pubkey_hash);
    let _cli:CLI = CLI{ blockchain: _blockchain };
    _cli.run();
}
