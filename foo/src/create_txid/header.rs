
extern crate secp256k1;
extern crate hex;
use std::{hash, str};
use sha2::{Digest, Sha256};

fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}

pub fn header(result_bytes: String) -> String {
    let mut raw_transaction = String::new();

    raw_transaction += "04000000";

    raw_transaction += "0000000000000000000000000000000000000000000000000000000000000000";
    raw_transaction += result_bytes.as_str();

    let temp= 1713342076;
    let temp = format!("{:08x}", temp);
    println!("temp is {}",temp);
    raw_transaction += &hex_to_little_endian(&temp);

    raw_transaction += "1f00ffff";
    raw_transaction += "00000000";

    raw_transaction
}


