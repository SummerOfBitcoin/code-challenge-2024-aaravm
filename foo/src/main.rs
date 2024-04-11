extern crate secp256k1;
extern crate hex;

use secp256k1::{Secp256k1, Message, PublicKey, Signature, All, Error};
use hex::{decode, FromHex};

use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use std::str;

fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}

fn create_transaction(data: serde_json::Value) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&input_count);

    for input in data["vin"].as_array().unwrap() {
        let prev_txid = input["txid"].as_str().unwrap();
        raw_transaction += &hex_to_little_endian(prev_txid);

        let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
        raw_transaction += &hex_to_little_endian(&prev_index);

        let script_pubkey_length = input["prevout"]["scriptpubkey"].as_str().unwrap().len() / 2;
        let script_pubkey_length_hex = format!("{:02x}", script_pubkey_length);
        raw_transaction += &hex_to_little_endian(&script_pubkey_length_hex);

        let script_pubkey = input["prevout"]["scriptpubkey"].as_str().unwrap();
        raw_transaction += script_pubkey;

        raw_transaction += "ffffffff";
    }

    let output_count = format!("{:02x}", data["vout"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&output_count);

    for output in data["vout"].as_array().unwrap() {
        let value = format!("{:016x}", (output["value"].as_f64().unwrap()) as u64);
        raw_transaction += &hex_to_little_endian(&value);

        let script_length = output["scriptpubkey"].as_str().unwrap().len() / 2;
        let script_length_hex = format!("{:02x}", script_length);
        raw_transaction += &hex_to_little_endian(&script_length_hex);

        let script_pubkey = output["scriptpubkey"].as_str().unwrap();
        raw_transaction += script_pubkey;
    }

    let locktime = format!("{:08x}", data["locktime"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&locktime);

    let sighash_all = "01000000";
    raw_transaction += sighash_all;


    let data1 = Vec::from_hex(raw_transaction).unwrap();
    let sha256_hash1 = Sha256::digest(&data1).to_vec();

    let data2 = sha256_hash1;
    let hash_hex1 = Sha256::digest(&data2).to_vec();

    let hex = hex::encode(hash_hex1);
    hex
}

fn main() {
    let mut f = File::open("../mempool/0ac528562a1626863c0cb912eb725530c54e786e6485380c16633e4b9bce1720.json").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let data: serde_json::Value = serde_json::from_str(&data).unwrap();

    let hex= create_transaction(data.clone());
    let hash_hex: &str = hex.as_str();

    let secp = Secp256k1::new();
    
    // Decode the signature, public key, and hash
    let signature = data["vin"][0]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(1).unwrap();
    let signature = &signature[..signature.len() - 2];
    let signature_bytes = decode(signature).expect("Failed to decode signature hex");

    let pub_key = data["vin"][0]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap();
    let pubkey_bytes = decode(pub_key).expect("Failed to decode pubkey hex");
    let pubkey = PublicKey::from_slice(&pubkey_bytes).expect("Invalid public key");

    // let hash_hex = "713f55b5ea939f8269a0757a86df761a7a0ddaca9e2f5d6cf761cf43fdf7e6f9";
    let hash_bytes = decode(hash_hex).expect("Failed to decode hash hex");
    let message = Message::from_slice(&hash_bytes).expect("Invalid message");

    // Create a signature object
    let signature = Signature::from_der(&signature_bytes).expect("Fuck! Invalid signature");

    // Verify the signature
    match secp.verify(&message, &signature, &pubkey) {
        Ok(_) => println!("Signature is valid!"),
        Err(Error::IncorrectSignature) => println!("Signature is invalid!"),
        _ => println!("Failed to verify signature!"),
    }
}
