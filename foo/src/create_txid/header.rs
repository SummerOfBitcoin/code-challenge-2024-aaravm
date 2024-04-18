
extern crate secp256k1;
extern crate hex;
use std::{hash, str};
use sha2::{Digest, Sha256};

// Utility function to hash data with SHA-256 twice
fn hash256(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let hash1 = hasher.finalize();
    
    let mut hasher = Sha256::new();
    hasher.update(hash1);
    let hash2 = hasher.finalize();
    
    hash2.to_vec()
}
// Utility function to convert a number to fit inside a field that is a specific number of bytes
fn field(data: u32, size: usize) -> Vec<u8> {
    let mut field = vec![0u8; size];
    let data_bytes = data.to_le_bytes();
    field.copy_from_slice(&data_bytes[..size]);
    field
}

fn reverse_bytes(data: &[u8]) -> Vec<u8> {
    data.iter().rev().copied().collect()
}

fn u256_from_bytes_be(bytes: &[u8]) -> num_bigint::BigUint {
    num_bigint::BigUint::from_bytes_be(bytes)
}

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

    let temp= 1713454776;
    let temp = format!("{:08x}", temp);
    println!("temp is {}",temp);
    raw_transaction += &hex_to_little_endian(&temp);

    raw_transaction += "1f00ffff";
    raw_transaction += "00000000";

    let version: u32 = 4;
    let prevblock = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let merkleroot = result_bytes.as_bytes().to_vec();
    let time: u32 = 1713454776;
    let bits = hex::decode("1f00ffff").unwrap();
    let mut nonce: u32 = 0;
;
    let mut header = Vec::new();
    header.extend_from_slice(&field(version, 4));
    header.extend_from_slice(&reverse_bytes(&prevblock));
    header.extend_from_slice(&reverse_bytes(&merkleroot));
    header.extend_from_slice(&field(time, 4));
    header.extend_from_slice(&reverse_bytes(&bits));

    let target = hex::decode("0000ffff00000000000000000000000000000000000000000000000000000000").unwrap();

    loop {
        // Hash the block header
        let mut attempt = header.clone();
        attempt.extend_from_slice(&field(nonce, 4));

        println!("{}",hex::encode(&attempt));
        let result = hash256(&attempt);

        // Show result
        println!("{:?}: {:?}", nonce, hex::encode(&result));
        

        // End if we get a block hash below the target
        if u256_from_bytes_be(&result) < u256_from_bytes_be(&target) {
            println!("{}",nonce);
            println!("{}",hex::encode(&attempt));

            break;
        }

        // Increment the nonce and try again
        nonce += 1;
    }



    raw_transaction
}


