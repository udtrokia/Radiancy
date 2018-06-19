
use blockchain::blockchain::Blockchain;
use tx::tx::Transaction;
use std::collections::HashMap;
use hex::encode;

impl Blockchain {
    pub fn sign_transaction(self, _tx: Transaction, _priv_key: Vec<u8>) {
        let mut _prev_txs:HashMap<String, Transaction> = HashMap::new();
        for _vin in _tx.vin.to_owned() {
            let _prev_tx = self.to_owned().find_transaction(_vin.txid);
            _prev_txs.insert(encode(_prev_tx.to_owned().id), _prev_tx);
        }

        _tx.sign(_prev_txs, _priv_key);
    }

    pub fn verify_transaction(self, _tx: Transaction) -> bool {
        let mut _prev_txs:HashMap<String, Transaction> = HashMap::new();
        for _vin in _tx.vin.to_owned() {
            let _prev_tx = self.to_owned().find_transaction(_vin.txid);
            _prev_txs.insert(encode(_prev_tx.to_owned().id), _prev_tx);
        }
        return _tx.verify(_prev_txs);
    }
}

