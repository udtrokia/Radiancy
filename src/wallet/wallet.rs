/*****
    *
  ** /radiancy/src/wallet/wallet.rs
  *
 */

//use std::collections::HashMap;
use ripemd160::Ripemd160;
use rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256,Digest};

// trate
use base58::{FromBase58, ToBase58};

// self
#[derive(Serialize, Deserialize, Clone)]
pub struct Wallet {
    priv_key: Vec<u8>,
    pub_key: Vec<u8>
}

impl Wallet {
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

pub fn hash_pubkey(pub_key: Vec<u8>) -> Vec<u8>{
    let mut hasher = Ripemd160::default();
    hasher.input(&pub_key);

    return hasher.result().to_vec();
}

pub fn check_sum(payload: Vec<u8>) -> Vec<u8> {
    let mut first_sha = Sha256::default();
    first_sha.input(&payload);
    let mut second_sha = Sha256::default();
    second_sha.input(&first_sha.result());
    
    return second_sha.result().to_vec();
}

pub fn new_keypair() -> (Vec<u8>, Vec<u8>){
    let curve = Secp256k1::new();
    let _priv_key = rand::random::<[u8;32]>();
    let _secret_key = SecretKey::from_slice(&curve, &_priv_key).unwrap();
    let _public_key = PublicKey::from_secret_key(&curve, &_secret_key).unwrap();
    return (_priv_key.to_vec(), _public_key.serialize().to_vec());
}

pub fn validate_address(address: String) -> bool {
    let mut pubkey_hash = address.from_base58().unwrap();
    let _actual_checksum = pubkey_hash[21..].to_vec();
    let mut _version = vec![pubkey_hash[0]];
    pubkey_hash = pubkey_hash[1..20].to_vec();
    _version.append(&mut pubkey_hash);

    let _target_checksum = check_sum(_version.to_owned());
    return _actual_checksum.eq(&_target_checksum);
}

pub fn new_wallet() -> Wallet {
    let (private, public) = new_keypair();
    let wallet = Wallet {
        priv_key: private,
        pub_key: public,
    };
    return wallet;
}
