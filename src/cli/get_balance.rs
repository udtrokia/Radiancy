use std::env;
use cli::cli::CLI;
use blockchain::utils::new_blockchain;

impl CLI {
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
}
