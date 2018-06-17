/*****
    *
  ** /radiancy/src/wallet/wallet.rs
  *
 */

use std::collections::HashMap;
use ripemd160::Ripemd160;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256,Digest};
// trate
use base58::ToBase58;

// self
#[derive(Clone)]
pub struct Wallet {
    priv_key: SecretKey,
    pub_key: PublicKey
}

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallet {
    fn get_address(self) -> Vec<u8> {
        let mut version = vec![00];
        let mut pub_key_hash = self.clone().hash_pubkey(self.pub_key.to_owned());
        version.append(&mut pub_key_hash);
        let mut checksum = self.clone().check_sum(version.to_owned());
        version.append(&mut checksum);

        let address = version.to_base58();
        return address.into_bytes();
    }

    pub fn hash_pubkey(self, pub_key: PublicKey) -> Vec<u8>{
        let mut hasher = Ripemd160::default();
        hasher.input(&pub_key.serialize());

        return hasher.result().to_vec();
    }

    pub fn check_sum(self, payload: Vec<u8>) -> Vec<u8> {
        let mut first_sha = Sha256::default();
        first_sha.input(&payload);
        let mut second_sha = Sha256::default();
        second_sha.input(&first_sha.result());
        
        return second_sha.result().to_vec();
    }
}

pub fn new_wallet() -> Wallet {
    let (private, public) = new_keypair();
    let wallet = Wallet {
        priv_key: private,
        pub_key: public,
    };
    return wallet;
}


pub fn new_keypair() -> (SecretKey, PublicKey){
    let curve = Secp256k1::new();
    let _private_key = SecretKey::from_slice(&curve, b"secret").unwrap();
    let _public_key = PublicKey::from_secret_key(&curve, &_private_key).unwrap();
    return (_private_key, _public_key);
}
