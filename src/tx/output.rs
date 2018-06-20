// TXOutput

use base58::{FromBase58};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TXOutput {
    pub value: i32,
    pub pubkey_hash: Vec<u8>, // just address now
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
