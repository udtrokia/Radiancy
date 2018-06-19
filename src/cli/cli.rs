/*****
    *
  ** /radiancy/src/cli/cli.rs
  *
 */

use std::env;
use blockchain::blockchain::{Blockchain};

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
}
