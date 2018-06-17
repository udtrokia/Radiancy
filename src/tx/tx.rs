// Tx
use bincode::serialize;
use sha2::{Sha256, Digest};
use blockchain::blockchain::{Blockchain};
use wallet::wallet::hash_pubkey;
use base58::{FromBase58};
    
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
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub reward: i32, // reward from blockchain
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXOutput {
    pub value: i32,
    pub pubkey_hash: Vec<u8>, // just address now
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
        let pubkey_hash = _address.from_base58().unwrap();
        return TXOutput{
            value: self.value,
            pubkey_hash: pubkey_hash,            
        }
    }

    pub fn islocked_with_key(self, _pubkey_hash: Vec<u8>) -> bool {
        return self.pubkey_hash.eq(&_pubkey_hash);
    }
}

pub fn new_coinbase_tx(to: String, mut data: String) -> Transaction {
    if data == "".to_string() {
        data = "Reward to ".to_string() + &to;
    }
    let subsidy = 1;
    let txin = TXInput {
        txid: vec![],
        reward: -1,
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
                reward: out.to_owned(),
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
    

    let mut _tx = Transaction{
        id: vec![],
        vin: _inputs,
        vout: _outputs,
    };
    _tx = _tx.set_id();
    
    return Some(_tx);
}
