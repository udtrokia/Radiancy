
use bincode::{serialize, deserialize};
use wallet::account::Account;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::io::Result;

#[derive(Serialize, Deserialize)]
pub struct Wallet {
    #[serde(with = "HashMap")]
    wallets: HashMap<String, Account>,
}

impl Wallet {
    pub fn new_wallets(self) -> Wallet {
        let wallets = self.load_from_file();
        return wallets;
    }

    pub fn get_addresses(self) -> Vec<String> {
        let mut addresses: Vec<String> = vec![];
        for address in self.wallets {
            addresses.append(&mut vec![address.0]);
        }

        return addresses;
    }
    
    pub fn get_account(self, address: String) -> Account {
        let _wallet = self.wallets[&address].to_owned();
        return _wallet;
    }
    
    pub fn load_from_file(mut self) -> Wallet {
        let mut f = File::open("wallets").unwrap();
        let mut _f_buffer = vec![];
        f.read_to_end(&mut _f_buffer).unwrap();
        
        let _wallets:Wallet = deserialize(&_f_buffer[..]).unwrap();
        self = _wallets;
        return self;
    }

    pub fn save_to_file(self) -> Result<()> {
        let mut buf = File::create("wallets")?;
        buf.write(&serialize(&self).unwrap())?;
        Ok(())
    }
}
