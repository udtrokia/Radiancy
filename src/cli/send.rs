use blockchain::utils::new_blockchain;
use cli::cli::CLI;
use std::env;
use tx::utils::{new_utxo_transaction};
use tx::utils::{new_coinbase_tx};
use tx::utxo_set::UTXOSet;
use wallet::utils::{hash_pubkey, load_account, address_to_pubkey_hash};


impl CLI {
    pub fn send(self) {
        if env::args().nth(2).is_none() {
            println!("\nPlease input `to` address.\n");
            return;
        }
        if env::args().nth(3).is_none() {
            println!("\nPlease input coin amount.\n");
            return;
        }
        
        let _to_arg = env::args().nth(2).unwrap().to_string();
        let _to = address_to_pubkey_hash(_to_arg);
        
        let _amount = env::args().nth(3).unwrap().parse::<i32>().unwrap();
        println!("\nsend out transaction...");

        let account = load_account();
        let pubkey_hash = hash_pubkey(account.pub_key);
        let _bc = new_blockchain(pubkey_hash.to_owned());
        let _utxo_set = UTXOSet{blockchain: _bc.to_owned()};
        let _tx = new_utxo_transaction(_to, pubkey_hash.to_owned(), _amount, _utxo_set.to_owned());
        if _tx.is_none() { println!("\nnot enough funds~\n");return;}

        let _cbtx = new_coinbase_tx(pubkey_hash, "".to_string());
        let _txs = vec![_cbtx, _tx.unwrap()];
        let new_block = _bc.mine_block(_txs);
        _utxo_set.update(new_block);
        println!("\n<-- Success -->!\n")
    }    
}
