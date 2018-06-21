use blockchain::blockchain::Blockchain;
use blockchain::block::Block;
use std::collections::HashMap;
use tx::outputs::TXOutputs;
use hex::{encode, decode};

#[derive(Clone)]
pub struct UTXOSet {
    pub blockchain: Blockchain
}

impl UTXOSet {
    pub fn re_index(self) {
        let _db = self.blockchain.state_db.to_owned();
        let _utxo = self.blockchain.find_utxo();
        
        for (_tx_id, outs) in _utxo {
            let key = decode(_tx_id);
            assert_eq!(key.is_ok(), true);
            let _res = _db.set(key.to_owned().unwrap(), outs.to_owned().s());
            assert_eq!(_res.is_ok(), true);
        }
    }

    pub fn find_spendable_outputs(self, _pubkey_hash: Vec<u8>, _amount: i32) -> (i32, HashMap<String, Vec<i32>>) {
        let mut _unspent_outputs:HashMap<String, Vec<i32>> = HashMap::new();
        let mut _accumulated = 0;
        let mut _db = self.blockchain.state_db.iter();
        
        for _res in _db.next() {
            assert_eq!(_res.is_ok(), true);
            let (_k, _v) = _db.next().unwrap().unwrap();
            let _tx_id = encode(_k);
            let _outs = TXOutputs::ds(_v);
            for (_out_idx, _out) in _outs.outputs.iter().enumerate() {
                if _out.to_owned().islocked_with_key(_pubkey_hash.to_owned()) && _accumulated < _amount {
                    _accumulated += _out.value;
                    _unspent_outputs.insert(_tx_id.to_owned(), vec![(_out_idx as i32)]);
                }
            }
        }
        return (_accumulated, _unspent_outputs);
    }

    pub fn find_utxo(self, _pubkey_hash: Vec<u8>) -> TXOutputs {
        let mut _utxos: TXOutputs = TXOutputs{outputs: vec![]};
        let mut _db = self.blockchain.state_db.iter();
        println!("count: {:?}", self.blockchain.to_owned().state_db.iter().count());
        for _res in _db {
            assert_eq!(_res.is_ok(), true);
            let (_k, _v) = _res.unwrap();
            let _outs = TXOutputs::ds(_v);
            println!("{:?}", &_outs);
            for out in _outs.outputs {
                println!("{:?}", out.to_owned());
                if out.to_owned().islocked_with_key(_pubkey_hash.to_owned()) {
                    _utxos.outputs.append(&mut vec![out]);
                }
            }
        }
        return _utxos;
    }

    pub fn update(self, _block: Block) {
        let _db = self.blockchain.state_db;
        for _tx in _block.transactions {
            if _tx.to_owned().is_coinbase() == false {
                for _vin in _tx.to_owned().vin {
                    let mut _updated_outs = TXOutputs{ outputs: vec![] };
                    assert_eq!(_db.get(&_vin.txid).is_ok(), true);
                    assert_eq!(_db.get(&_vin.txid).unwrap().is_some(), true);
                    let _outs_bytes = _db.get(&_vin.txid).unwrap().unwrap();
                    let _outs = TXOutputs::ds(_outs_bytes);
                    for (_out_idx, _out) in _outs.outputs.iter().enumerate() {
                        _updated_outs.outputs.append(&mut vec![_out.to_owned()]);
                    }
                }
            }

            let mut _new_outputs = TXOutputs{outputs: vec![]};
            for _out in _tx.to_owned().vout {
                _new_outputs.outputs.append(&mut vec![_out.to_owned()]);
            }
            let _res = _db.set(_tx.to_owned().id, _new_outputs.s());
            assert_eq!(_res.is_ok(), true);
        }
    }
}
