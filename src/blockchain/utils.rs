// utils

use blockchain::blockchain::Blockchain;
use sled::{ Iter };
use blockchain::block::{ Block, new_genesis_block };
use db::db::{block_db, state_db};
use tx::utils::new_coinbase_tx;
use tx::utxo_set::UTXOSet;

pub fn new_blockchain(pubkey_hash: Vec<u8>) -> Blockchain {
        
    let _db = block_db();
    let _state_db = state_db();
    let _dbc = block_db().clone();
    let tip: Vec<u8> = vec![];        
    let iter:Iter = _dbc.scan(b"no previous block...");
    
    let height = iter.count();
    let genesis_coinbase_data = "I'm genesis coinbase data".to_string();

    let mut _new_blockchain = Blockchain {
        tip: tip,
        block_db: block_db(),
        state_db: state_db(),
    };
    
    if height == 0 {
        let cbtx = new_coinbase_tx(pubkey_hash, genesis_coinbase_data);
        let genesis:Block = new_genesis_block(cbtx);
        
        let _store_hash = _db.set(genesis.clone().hash, genesis.clone().serialize());
        if _store_hash.is_ok() == false { panic!(_store_hash.unwrap())}
        
        let _store_last = _db.set("last".to_string().into_bytes(), genesis.clone().hash);
        if _store_last.is_ok() == false { panic!(_store_last.unwrap())};
        _new_blockchain.tip = genesis.to_owned().hash;

        let _utxo_set = UTXOSet{blockchain: _new_blockchain.to_owned()};
        _utxo_set.update(genesis);
    } else {
        _new_blockchain.tip = _db.get(&"last".to_string().into_bytes()).unwrap().unwrap().to_vec();
    }

        
    return _new_blockchain;
}
