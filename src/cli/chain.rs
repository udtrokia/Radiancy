use cli::cli::CLI;
use num_bigint::{BigInt, Sign};
use blockchain::utils::new_blockchain;
use pow::utils::{new_proof_of_work};
use wallet::utils::{hash_pubkey, load_account};

impl CLI {
    pub fn create_blockchain(self){
        let _account = load_account();
        let _pubkey_hash = hash_pubkey(_account.pub_key);
        new_blockchain(_pubkey_hash);

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
}
