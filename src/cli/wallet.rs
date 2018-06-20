use blockchain::utils::new_blockchain;
use cli::cli::CLI;
use std::env;
use tx::utils::{new_utxo_transaction};
use tx::utils::{new_coinbase_tx};
use tx::utxo_set::UTXOSet;
use wallet::utils::{new_wallet};


impl CLI {
    pub fn create_wallet(self){
        let wallet = new_wallet();
        let _address = wallet.get_address();
        
        println!("\n{:?}", String::from_utf8(_address).unwrap());
        println!("\nSuccess!\n");
    }
    
    pub fn get_balance(self) {
        if env::args().nth(2).is_none() {
            println!("\nPlease input address name\n");
            return;
        }
        
        let address = env::args().nth(2).unwrap().to_string();
        println!("\nlink blockchain...");
        let _bc = new_blockchain(address.to_owned());
        
        let mut balance = 0;
        println!("\nfind utxos...");
        let utxos = _bc.find_utxo();
    
        println!("\nsum utxos...");
        for tx_outputs in utxos.values() {
            //println!("{:?}", out.to_owned());
            for output in tx_outputs.to_owned().outputs {
                balance = balance + &output.value;
            }
        }
        
        println!("\nBalance of {:?}: {:?}\n", address, balance);
    }
    
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
