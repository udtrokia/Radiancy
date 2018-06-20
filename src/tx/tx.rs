// Tx
use bincode::serialize;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use secp256k1::{SecretKey, Secp256k1, Message, PublicKey};
use secp256k1::schnorr::Signature;
use hex::encode;
use tx::input::TXInput;
pub use tx::output::TXOutput;


#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Vec<u8>,
    pub vin: Vec<TXInput>, //TXInput,
    pub vout: Vec<TXOutput>, //TXOutput,
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



