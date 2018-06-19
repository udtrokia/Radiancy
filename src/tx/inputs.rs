
use wallet::wallet::hash_pubkey;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXInput {
    pub txid: Vec<u8>,
    pub vout_idx: i32, // vout_idx from blockchain
    pub signature: Vec<u8>,
    pub pub_key: Vec<u8>
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
