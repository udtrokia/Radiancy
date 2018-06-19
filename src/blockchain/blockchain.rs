/*****
 *
 ** /radiancy/src/blockchain/blockchain.rs
 * 
 */

use blockchain::block::{ Block, new_block };
use blockchain::iterator::{ Iterator as BlockchainIterator };
use db::db::{Tree, block_db};
use tx::tx::{Transaction};


#[derive(Clone)]
pub struct Blockchain {
    pub tip: Vec<u8>,
    pub block_db: Tree,
    pub state_db: Tree,
}

impl Blockchain {
    pub fn iterator(self) -> BlockchainIterator {
        let bci = BlockchainIterator {
            current_hash: self.clone().tip,
            block_db: self.block_db
        };
        return bci;
    }
    
    pub fn mine_block(self, transactions: Vec<Transaction>) {
        let _db = block_db();
        let last_hash: Vec<u8> = self.clone()
            .block_db.get(&"last".to_string().into_bytes()).unwrap().unwrap();

        for _tx in transactions.to_owned() {
            if self.to_owned().verify_transaction(_tx) == false {
                panic!("ERROR: Invalid transaction");
            }
        }
        
        let new_block: Block = new_block(transactions, last_hash);
        
        let _set_hash = self.block_db.set(new_block.clone().hash, new_block.clone().serialize());
        if _set_hash.is_ok() == false { panic!(_set_hash.unwrap()) };
        let _set_last = self.block_db.set("last".to_string().into_bytes(), new_block.clone().hash);
        if _set_last.is_ok() == false { panic!(_set_last.unwrap()) };
    }
}


