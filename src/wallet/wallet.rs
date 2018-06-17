/*****
    *
  ** /radiancy/src/wallet/wallet.rs
  *
 */

//use crypto::ripemd160::Ripemd160;

use std::collections::HashMap;
use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct Wallet {
    private_key: SecretKey,
    public_key: PublicKey
}

pub struct Wallets {
    wallets: HashMap<String, Wallet>,
}

pub fn new_wallet() -> Wallet {
    let (private, public) = new_keypair();
    let wallet = Wallet {
        private_key: private,
        public_key: public,
    };
    return wallet;
}

pub fn new_keypair() -> (SecretKey, PublicKey){
    let curve = Secp256k1::new();
    let _private_key = SecretKey::from_slice(&curve, b"secret").unwrap();
    let _public_key = PublicKey::from_secret_key(&curve, &_private_key).unwrap();
    return (_private_key, _public_key);
}
