/*****
    *
  ** /radiancy/src/cli/cli.rs
  *
 */

use std::env;
use blockchain::blockchain::{Blockchain, new_blockchain};
use num_bigint::{BigInt, Sign};
use pow::pow::{new_proof_of_work};
use tx::tx::{new_utxo_transaction};
use wallet::wallet::{new_wallet};

pub struct CLI {
    pub blockchain: Blockchain
}

impl CLI {
    pub fn run(self) {
        match env::args().nth(1) {
            Some(_) => {},
            None => { self.help(); return;}
        }
        let _arg = env::args().nth(1).unwrap();
        match _arg.as_str() {
            "create_blockchain" => {self.create_blockchain(); },
            "create_wallet" => {self.create_wallet(); },            
            "help" => { self.help(); },
            "get_balance" => { self.get_balance(); },
            "print" => { self.print_chain(); },
            "send" => { self.send(); },
            _ => { self.help() }
        }
    }
    pub fn help(self) {
        println!("\n<-- Hello Yellow Brick Road -->");        
        println!("\nUsage: radiancy COMMAND;");
        println!("\nCOMMANDS:");
        println!("    create_blockchain         Generate a blockchain;");
        println!("    create_wallet             Generate a wallet;");
        println!("    get_balance               Get address balance;");
        println!("    print                     Print blocks in Radiancy;");
        println!("    send                      Send coin between addresses;");
        println!("\n<-- GoodBye Yellow Brick Road -->");
        println!("");
    }
    
    pub fn create_blockchain(self){
        if env::args().nth(2).is_none() {
            println!("Please input address");
            return;
        }
        let address = env::args().nth(2).unwrap();
        new_blockchain(address.to_string());

        println!("\nSuccess!\n");
    }

    pub fn create_wallet(self){
        let wallet = new_wallet();
        let _address = wallet.get_address();
        
        println!("\n{:?}", String::from_utf8(_address).unwrap());
        println!("\nSuccess!\n");
    }
    
    pub fn print_chain(self) {
        let mut _bci = self.blockchain.iterator();
        println!("\n<-- Start -->\n");
        loop {
            let (_new_bci, _block) = _bci.clone().next();
            _bci = _new_bci;
            
            println!("\nPrev. hash: {:x}", BigInt::from_bytes_be(Sign::Plus,&_block.prev_block_hash));
            println!("Hash: {:x}", BigInt::from_bytes_be(Sign::Plus, &_block.hash));
            println!("Time: {}", String::from_utf8(_block.clone().timestamp).unwrap());
            let pow = new_proof_of_work(_block.clone(), 0);
            println!("PoW:  {}\n", pow.validate().to_string());
            if _block.prev_block_hash == vec![] {
                println!(" <-- Complete! -->\n");
                break;
            };
        };
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
        let utxos = _bc.find_utxo(address.to_owned());

        println!("\nsum utxos...");
        for out in utxos {
            println!("{:?}", out);
            balance = balance + &out.value;
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
        let _tx = new_utxo_transaction(_to, _from.to_owned(), _amount, _bc.to_owned());
        if _tx.is_none() { println!("\nnot enough funds~\n");return;}

        _bc.mine_block(vec![_tx.unwrap()]);
        println!("\n<-- Success -->!\n")
    }
}







