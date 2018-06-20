/*****
    *
  ** /radiancy/src/wallet/wallet.rs
  *
 */

use base58::ToBase58;
use wallet::utils::{
    hash_pubkey, check_sum,
};

// self
#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub priv_key: Vec<u8>,
    pub pub_key: Vec<u8>
}

impl Account {
    pub fn get_address(self) -> Vec<u8> {
        let mut version = vec![00];
        let mut pubkey_hash = hash_pubkey(self.pub_key.to_owned());
        version.append(&mut pubkey_hash);
        
        let mut checksum = check_sum(version.to_owned());
        version.append(&mut checksum);
        
        let address = version.to_base58();
        return address.into_bytes();
    }
}
