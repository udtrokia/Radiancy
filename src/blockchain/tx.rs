use blockchain::blockchain::Blockchain;
use tx::tx::Transaction;
use tx::outputs::{TXOutputs};
use std::collections::HashMap;
use hex::encode;
use blockchain::iterator::{ Iterator as BlockchainIterator };

impl Blockchain {
    pub fn find_transaction(self, _id: Vec<u8>) -> Transaction {
        println!("find_transaction...");
        let mut _bci = self.iterator();
        loop {
            let (_new_bci, _block) = _bci.next();
            _bci = _new_bci;
            
            for _tx in _block.transactions {
                if _tx.id.eq(&_id) == true { return _tx };
            }            
            if _block.prev_block_hash.len() == 0 { break; };
        }
        return Transaction{ id: vec![], vin: vec![], vout: vec![] };
    }

    pub fn find_utxo(self) -> HashMap<String, TXOutputs> {
        println!("find_utxo.....");
        let mut _utxo:HashMap<String, TXOutputs> = HashMap::new();
        let mut spent_txos:HashMap<String, Vec<i32>> = HashMap::new();
        let mut bci: BlockchainIterator = self.iterator();
        
        loop {
            let (_new_bci, _block) = bci.clone().next();
            bci = _new_bci;
            
            for tx in _block.transactions {
                let _tx_id = encode(tx.clone().id);
                
                'outputs: for (_out_idx, _out) in tx.vout.iter().enumerate() {
                    if spent_txos.get(&_tx_id).is_some() && spent_txos[&_tx_id] != vec![] {
                        for spent_out_idx in &spent_txos[&_tx_id] {
                            if spent_out_idx == &(_out_idx as i32) { continue 'outputs; };
                        }
                        
                        let mut _outs = _utxo[&_tx_id].to_owned();
                        _outs.outputs.append(&mut vec![_out.clone()]);
                        _utxo.insert(_tx_id.to_owned(), _outs);
                    }
                    
                    if tx.to_owned().is_coinbase() == false {
                        for _vin in tx.to_owned().vin {
                            let _in_txid = encode(_vin.txid);
                            spent_txos.insert(_in_txid, vec![_vin.vout_idx]);
                        }
                    }
                }
            }
            if _block.prev_block_hash.len() == 0 {break;};
        }
        return _utxo;
    }    
}
