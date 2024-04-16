
extern crate secp256k1;
extern crate hex;
use std::{hash, str};
use sha2::{Digest, Sha256};

fn hash256(hex: &str) -> String {
    let binary = hex::decode(hex).unwrap();
    let hash1 = Sha256::digest(&binary);
    let hash2 = Sha256::digest(&hash1);
    let result = hex::encode(hash2);
    result
}

fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}

pub fn coinbase(result_bytes: String) -> String {
    let mut raw_transaction = String::new();

    raw_transaction += "04000000";

    raw_transaction += "00";
    raw_transaction += "01";

    raw_transaction += "01";

    raw_transaction += "0000000000000000000000000000000000000000000000000000000000000000";
    raw_transaction += "ffffffff";
    
    let temp = 900010;
    let temp = format!("{:06x}", temp);
    let mut temp_string = String::new();
    temp_string += "03";
    temp_string += &hex_to_little_endian(&temp);

    temp_string += "184d696e656420627920416e74506f6f6c373946205b8160a4256c0000946e0100";

    // println!("temp_string is {}",temp_string);

    let tem_temp = temp_string.len() / 2;
    let tem_temp = format!("{:02x}", tem_temp);
    raw_transaction += &hex_to_little_endian(&tem_temp);
    raw_transaction += &temp_string;

    raw_transaction += "ffffffff";
    raw_transaction += "02";

    raw_transaction += "00000000000000001976a914edf10a7fac6b32e24daa5305c723f3de58db1bc888ac0000000000000000";

    let mut x = String::new();
    x += result_bytes.as_str();
    x += "0000000000000000000000000000000000000000000000000000000000000000";

    raw_transaction += "26";
    raw_transaction += "6a24aa21a9ed";
    raw_transaction += &hash256(&x);

    raw_transaction += "0120000000000000000000000000000000000000000000000000000000000000000000000000";

    raw_transaction
}


