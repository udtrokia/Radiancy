use std::env;
use blockchain::blockchain::Blockchain;
use pow::pow::{new_proof_of_work};
use num_bigint::{BigInt, Sign};


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
            "add" => { self.add_block(); },
            "print" => { self.print_chain(); },
            _ => println!("no match"),
        }
    }
    pub fn help(self) {
        println!("\nUsage: radiancy COMMAND;");
        println!("\n<--Yellow Brick Road-->");
        println!("\nCOMMANDS:");
        println!("    add        Add block to Radiancy;");
        println!("    print      Print blocks in Radiancy;");
        println!("");
    }
    pub fn add_block(self) {
        let _data = env::args().nth(2).unwrap();
        self.blockchain.add_block(_data);
        println!("Success!\n");
    }
    pub fn print_chain(self) {
        let mut _bci = self.blockchain.iterator();
        loop {
            let (_new_bci, _block) = _bci.clone().next();
            _bci = _new_bci;

            println!("\nPrev. hash: {:x}", BigInt::from_bytes_be(Sign::Plus,&_block.prev_block_hash));
            println!("Data: {}", String::from_utf8(_block.data.to_vec()).unwrap());
            println!("Hash: {:x}", BigInt::from_bytes_be(Sign::Plus, &_block.hash));
            println!("Time: {}", String::from_utf8(_block.clone().timestamp).unwrap());
            let pow = new_proof_of_work(_block.clone(), 0);
            println!("PoW:  {}\n", pow.validate().to_string());
            if _block.prev_block_hash == vec![] {
                println!(" <-- Complete! --> ");
                break;
            };
        }; 
    }
}
