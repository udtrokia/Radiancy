// Tx
use bincode::serialize;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use blockchain::blockchain::{Blockchain};
use wallet::wallet::hash_pubkey;
use base58::{FromBase58};
use secp256k1::{SecretKey, Secp256k1, Message, PublicKey};
use secp256k1::schnorr::Signature;
use hex::encode;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>, //TXInput,
    pub vout: Vec<TXOutput>, //TXOutput,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout_idx: i32, // vout_idx from blockchain
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXOutput {
    pub value: i32,
    pub pubkey_hash: Vec<u8>, // just address now
}

impl Transaction {
    pub fn is_coinbase(self) -> bool {
        if self.id == vec![] {
            return true;
        }else{
            return false;
        }
    }

    pub fn set_id(self) -> Transaction {
        let _encoder = serialize(&self.clone());
        let mut _hasher = Sha256::default();
        _hasher.input(&_encoder.unwrap());
        let _hasher_result = _hasher.result().to_vec();
        return Transaction {
            id: _hasher_result,
            vin: self.vin.to_owned(),
            vout: self.vout.to_owned()
        };
    }

    pub fn sign(mut self, _prev_txs: HashMap<String, Transaction>, _priv_key: Vec<u8>) {
        if self.to_owned().is_coinbase(){return;};
        let mut _tx_copy = self.clone().trimmed_copy();
        let _secp = Secp256k1::new();
        
        for (_in_id, _vin) in _tx_copy.vin.to_owned().iter().enumerate() {
            let _prev_tx = _prev_txs[&encode(_vin.to_owned().txid)].to_owned();
            _tx_copy.vin[_in_id].signature = vec![];
            _tx_copy.vin[_in_id].pub_key = _prev_tx.vout[(_vin.vout_idx as usize)].pubkey_hash.to_owned();

            let mut hasher = Sha256::new();
            hasher.input(&serialize(&_tx_copy).unwrap());
            _tx_copy.id = hasher.result().to_vec();
            _tx_copy.vin[_in_id].pub_key = vec![];

            let _message = Message::from_slice(&_tx_copy.id).unwrap();
            let _secret_key = SecretKey::from_slice(&_secp, &_priv_key).unwrap();
            let _signature = _secp.sign_schnorr(&_message, &_secret_key).unwrap().serialize();
            self.vin[_in_id].signature = _signature;
        }
    }

    pub fn verify(self, _prev_txs: HashMap<String, Transaction>) -> bool {
        let mut _tx_copy = self.to_owned().trimmed_copy();
        let _secp = Secp256k1::new();
        
        for (_in_id, _vin) in self.to_owned().vin.iter().enumerate() {
            let _prev_tx = _prev_txs[&encode(_vin.to_owned().txid)].to_owned();
            _tx_copy.vin[_in_id].signature = vec![];
            _tx_copy.vin[_in_id].pub_key = _prev_tx.vout[(_vin.vout_idx as usize)].pubkey_hash.to_owned();

            let mut hasher = Sha256::new();
            hasher.input(&serialize(&_tx_copy).unwrap());
            _tx_copy.id = hasher.result().to_vec();
            _tx_copy.vin[_in_id].pub_key = vec![];
            
            let _msg = Message::from_slice(&_tx_copy.id).unwrap();
            let _sig = &self.vin[_in_id].signature;
            
            let _raw_sig = Signature::deserialize(_sig); 
            let _pub_key = PublicKey::from_slice(&_secp, &_vin.pub_key).unwrap();
            let _verify = _secp.verify_schnorr(&_msg, &_raw_sig, &_pub_key).is_ok();

            if _verify == false { return false; };
        }
        return true;
        
    }

    pub fn trimmed_copy(self) -> Transaction {
        let mut _inputs: Vec<TXInput> = vec![];
        let mut _outputs: Vec<TXOutput> = vec![];

        for vin in self.vin {
            _inputs.append( &mut vec![TXInput{
                txid: vin.txid.to_owned(),
                vout_idx: vin.vout_idx.to_owned(),
                signature: vin.signature,
                pub_key: vin.pub_key
            }]);
        }

        for vout in self.vout {
            _outputs.append( &mut vec![TXOutput{
                value: vout.value,
                pubkey_hash: vout.pubkey_hash
            }]);
        }

        let _tx_copy = Transaction{
            id: self.id,
            vin: _inputs,
            vout: _outputs,
        };

        return _tx_copy;}
}

impl TXInput {
    pub fn can_unlock_output_with(self, unlocking_data: String) -> bool {
        return self.signature == unlocking_data.into_bytes();
    }

    pub fn uses_key(self, pubkey_hash: Vec<u8>) -> bool {
        let locking_hash = hash_pubkey(self.pub_key);
        
        return locking_hash.eq(&pubkey_hash);
    }
}

impl TXOutput {
    pub fn can_be_unlocked_with(self, unlocking_data: String) -> bool {
        return self.pubkey_hash == unlocking_data.into_bytes();
    }

    pub fn lock(self, _address: String) -> TXOutput {
        let pubkey_hash = _address.from_base58()
            .unwrap()[1..20].to_vec();
        return TXOutput{
            value: self.value,
            pubkey_hash: pubkey_hash,
        }
    }

    pub fn islocked_with_key(self, _pubkey_hash: Vec<u8>) -> bool {
        return self.pubkey_hash.eq(&_pubkey_hash);
    }
}

pub fn new_coinbase_tx(to: String, data: String) -> Transaction {
    println!("{:?}", data);
    let subsidy = 1;
    let txin = TXInput {
        txid: vec![],
        vout_idx: -1,
        signature: vec![],
        pub_key: vec![]
    };
    let txout = TXOutput{
        value: subsidy,
        pubkey_hash: to.into_bytes(),
    };
    let mut tx = Transaction {
        id: vec![],
        vin: vec![txin],
        vout: vec![txout]
    };
    tx = tx.set_id();

    return tx;
}

pub fn new_utxo_transaction(_to: String, _from: String, _amount: i32, _bc: Blockchain) -> Option<Transaction> {
    let mut _inputs: Vec<TXInput> = vec![];
    let mut _outputs: Vec<TXOutput> = vec![];
    let (_acc, _valid_outputs) = _bc.find_spendable_outputs(_from.to_owned(), _amount);

    for (_txid, _outs) in _valid_outputs.clone().iter() {
        let _tx_id = _txid.to_owned().into_bytes();
        
        for out in _outs {
            let _input = TXInput{
                txid: _tx_id.to_owned(),
                vout_idx: out.to_owned(),
                signature: _from.to_owned().into_bytes(),
                pub_key: vec![]
            };
            _inputs.append(&mut vec![_input]);
        }
    }

    if _acc < _amount { return None; }

    _outputs.append(&mut vec![TXOutput{
        value: _amount,
        pubkey_hash: _to.into_bytes()
    }]);
    
    
    _outputs.append(&mut vec![TXOutput{
        value:  -_amount,
        pubkey_hash: _from.into_bytes()
    }]);        
    

    let mut _tx = Transaction{ id: vec![], vin: _inputs, vout: _outputs };
    _tx = _tx.set_id();
    //_bc.sign_transaction(_tx.to_owned(), wallet.priv_key);
    
    return Some(_tx);
}
