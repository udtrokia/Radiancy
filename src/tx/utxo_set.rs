use blockchain::blockchain::Blockchain;

pub struct UTXOSet {
    blockchain: Blockchain
}

impl UTXOSet {
    pub fn re_index(self) {
        let _db = self.blockchain.state_db;
        //let utxo = self.blockchain.find_utxo();
        
    }
}
