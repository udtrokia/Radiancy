/*****
    *
  ** /radiancy/src/wallet/wallet.rs
  *
 */

use std::collections::HashMap;
use ripemd160::Ripemd160;
use rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256,Digest};

// trate
use base58::ToBase58;

// self
#[derive(Clone)]
pub struct Wallet {
    priv_key: SecretKey,
    pub_key: Vec<u8>
}

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

impl Wallet {
    pub fn get_address(self) -> Vec<u8> {
        let mut version = vec![00];
        let mut pub_key_hash = hash_pubkey(self.pub_key.to_owned());
        version.append(&mut pub_key_hash);
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

pub fn new_keypair() -> (SecretKey, Vec<u8>){
    let curve = Secp256k1::new();
    //let _signature = [0, 0, 0, 0, 0, 0, 0, 0,
    //                  0, 0, 0, 0, 0, 0, 0, 0,
    //                  0, 0, 0, 0, 0, 0, 0, 0,
    //                  0, 0, 0, 0, 0, 0, 0, 1];
    let _signature = rand::random::<[u8;32]>();
    let _private_key = SecretKey::from_slice(&curve, &_signature).unwrap();
    let _public_key = PublicKey::from_secret_key(&curve, &_private_key).unwrap();
    return (_private_key, _public_key.serialize().to_vec());
}

pub fn new_wallet() -> Wallet {
    let (private, public) = new_keypair();
    let wallet = Wallet {
        priv_key: private,
        pub_key: public,
    };
    return wallet;
}
