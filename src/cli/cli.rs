/*****
    *
  ** /radiancy/src/cli/cli.rs
  *
 */

use std::env;
use blockchain::blockchain::Blockchain;

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
            "help" => { self.help(); },
            "print_chain" => { self.print_chain(); },
            "print_address" => { self._print_address(); },
            "create_account" => { self._create_account(); },
            "create_blockchain" => {self.create_blockchain(); },
            "get_balance" => { self.get_balance(); },
            "send" => { self.send(); },
            _ => { self.help(); }
        }
    }
}
