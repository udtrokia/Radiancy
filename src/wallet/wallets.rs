
use bincode::{serialize, deserialize};
use wallet::wallet::{Wallet};
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;
use std::io::Result;

#[derive(Serialize, Deserialize)]
pub struct Wallets {
    #[serde(with = "HashMap")]
    wallets: HashMap<String, Wallet>,
}

impl Wallets {
    pub fn new_wallets() {
        
    }

    pub fn load_from_file(mut self) -> Result<()> {
        let mut f = File::open("wallets")?;
        let mut _f_buffer = vec![];
        f.read_to_end(&mut _f_buffer)?;
        
        let _wallets:Wallets = deserialize(&_f_buffer[..]).unwrap();
        self = _wallets;
        Ok(())
    }

    pub fn save_to_file(self) -> Result<()> {
        let mut buf = File::create("wallets")?;
        buf.write(&serialize(&self).unwrap())?;
        Ok(())
    }
}
