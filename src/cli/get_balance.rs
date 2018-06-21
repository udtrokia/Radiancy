use cli::cli::CLI;
use blockchain::utils::new_blockchain;
use wallet::account::Account;
use wallet::utils::{load_account, hash_pubkey};
use tx::utxo_set::UTXOSet;

impl CLI {
    pub fn get_balance(self) {
        let _account: Account = load_account();
        let _pubkey_hash = hash_pubkey(_account.to_owned().pub_key);
        println!("\nlink blockchain...");
        let _bc = new_blockchain(_pubkey_hash.to_owned());
        let _utxo_set = UTXOSet{blockchain: _bc.to_owned()};
        _utxo_set.to_owned().re_index();
        let mut balance = 0;
        println!("\nfind utxos...");
        let utxos = _utxo_set.find_utxo(_pubkey_hash.to_owned());

        println!("\nsum utxos...");
        for output in utxos.outputs {
                balance = balance + &output.value;
        }
        let address = String::from_utf8(_account.get_address()).unwrap();
        println!("\nBalance of {:?}: {:?}\n", address, balance);
    }
}
