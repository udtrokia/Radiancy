use cli::cli::CLI;
use blockchain::utils::new_blockchain;
use wallet::account::Account;
use wallet::utils::{load_account, hash_pubkey, address_to_pubkey_hash};
use tx::utxo_set::UTXOSet;
use std::env;

impl CLI {
    pub fn get_balance(self) {

        let _account: Account = load_account();
        let mut _pubkey_hash = hash_pubkey(_account.to_owned().pub_key);        
        let mut address = String::from_utf8(_account.get_address()).unwrap();
        
        if env::args().nth(2).is_some() {
            address = env::args().nth(2).unwrap();
            _pubkey_hash =  address_to_pubkey_hash(env::args().nth(2).unwrap());
        }
        
        println!("\nlink blockchain...");
        let _bc = new_blockchain(_pubkey_hash.to_owned());
        let _utxo_set = UTXOSet{blockchain: _bc.to_owned()};

        println!("\nfind utxos...");        
        _utxo_set.to_owned().re_index();
        let utxos = _utxo_set.find_utxo(_pubkey_hash.to_owned());

        println!("\nsum utxos...");
        let mut balance = 0;
        for output in utxos.outputs {
                balance = balance + &output.value;
        }
        
        println!("\nBalance of {:?}: {:?}\n", address, balance);
    }
}
