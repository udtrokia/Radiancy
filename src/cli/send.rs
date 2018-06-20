use blockchain::utils::new_blockchain;
use cli::cli::CLI;
use std::env;
use tx::utils::{new_utxo_transaction};
use tx::utils::{new_coinbase_tx};
use tx::utxo_set::UTXOSet;

impl CLI {
    pub fn send(self) {
        if env::args().nth(2).is_none() {
            println!("\nPlease input `to` address.\n");
            return;
        }
        if env::args().nth(3).is_none() {
            println!("\nPlease input `from` address.\n");
            return;
        }
        if env::args().nth(4).is_none() {
            println!("\nPlease input coin amount.\n");
            return;
        }
        let _to = env::args().nth(2).unwrap().to_string();
        let _from = env::args().nth(3).unwrap().to_string();
        let _amount = env::args().nth(4).unwrap().parse::<i32>().unwrap();
        println!("\nsend out transaction...");
        
        let _bc = new_blockchain(_from.to_owned());
        let _utxo_set = UTXOSet{blockchain: _bc.to_owned()};
        let _tx = new_utxo_transaction(_to, _from.to_owned(), _amount, _utxo_set.to_owned());
        if _tx.is_none() { println!("\nnot enough funds~\n");return;}        

        let _cbtx = new_coinbase_tx(_from, "".to_string());
        let _txs = vec![_cbtx, _tx.unwrap()];
        let new_block = _bc.mine_block(_txs);
        _utxo_set.update(new_block);
        println!("\n<-- Success -->!\n")
    }    
}
