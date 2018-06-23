use base58::{FromBase58, ToBase58};
use ripemd160::Ripemd160;
use rand;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use sha2::{Sha256,Digest};
use wallet::account::Account;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use bincode::{serialize, deserialize};

// account
pub fn new_account() -> Account {
    let (private, public) = new_keypair();
    let account = Account {
        priv_key: private,
        pub_key: public,
    };
    return account;
}

pub fn create_account() {
    let mut path =  env::home_dir().unwrap();
    path.push(".radiancy/wallet/default-account.rdc");
    let _exist = File::open(path.to_owned()).is_ok();
    if _exist == true { return;};
    let buf = File::create(path);
    assert_eq!(buf.is_ok(), true);
    let mut _file = buf.unwrap();    
    let _res = _file.write( &serialize(&new_account()).unwrap() );
    assert_eq!(_res.is_ok(), true);
    println!("\ncreate account.... Ok(())");
}

pub fn load_account() -> Account {
    let mut path =  env::home_dir().unwrap();
    path.push(".radiancy/wallet/default-account.rdc");    
    let mut f = File::open(path).unwrap();
    let mut _f_buffer = vec![];
    f.read_to_end(&mut _f_buffer).unwrap();
    
    let _account:Account = deserialize(&_f_buffer[..]).unwrap();
    return _account;
}

// address
pub fn print_address() {
    println!("address: {:?}", String::from_utf8(new_account().get_address()).unwrap());
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


pub fn pubkey_hash_to_address(mut pubkey_hash: Vec<u8>) -> Vec<u8> {
    let mut version = vec![00];
    
    version.append(&mut pubkey_hash);
    let mut checksum = check_sum(version.to_owned());
    version.append(&mut checksum);

    let address = version.to_base58();
    return address.into_bytes()
}

pub fn address_to_pubkey_hash(address: String) -> Vec<u8> {
    let pubkey_hash = address.from_base58().unwrap();
    return pubkey_hash[1..20].to_vec();
}


// generator
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

