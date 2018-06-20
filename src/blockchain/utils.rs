// utils

use blockchain::blockchain::Blockchain;
use sled::{ Iter };
use blockchain::block::{ Block, new_genesis_block };
use db::db::{block_db, state_db};
use tx::utils::new_coinbase_tx;

pub fn new_blockchain(address:String) -> Blockchain {
    let _db = block_db();
    let _dbc = block_db().clone();
    let tip: Vec<u8>;
    let iter:Iter = _dbc.scan(b"no previous block...");
    
    let height = iter.count();
    let genesis_coinbase_data = "I'm genesis coinbase data".to_string();
    
    if height == 0 {
        let cbtx = new_coinbase_tx(address, genesis_coinbase_data);
        let genesis:Block = new_genesis_block(cbtx);
        
        let _store_hash = _db.set(genesis.clone().hash, genesis.clone().serialize());
        if _store_hash.is_ok() == false { panic!(_store_hash.unwrap())}
        let _store_last = _db.set("last".to_string().into_bytes(), genesis.clone().hash);
        if _store_last.is_ok() == false { panic!(_store_last.unwrap())}
        tip = genesis.hash;
    } else {
        tip = _db.get(&"last".to_string().into_bytes()).unwrap().unwrap().to_vec();
    }
    
    let _new_blockchain = Blockchain {
        tip: tip,
        block_db: block_db(),
        state_db: state_db(),
    };
    return _new_blockchain;
}
