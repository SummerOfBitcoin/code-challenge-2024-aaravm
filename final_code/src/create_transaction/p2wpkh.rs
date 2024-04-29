
extern crate secp256k1;
extern crate hex;
use hex::FromHex;
use sha2::{Digest, Sha256};
use std::str;


fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}


pub fn create_transaction_p2wpkh(data: serde_json::Value, parameter: usize) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let mut hash_prevouts = String::new();

    for input in data["vin"].as_array().unwrap() {
        let prev_txid = input["txid"].as_str().unwrap();
        hash_prevouts += &hex_to_little_endian(prev_txid);

        let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
        hash_prevouts += &hex_to_little_endian(&prev_index);
    }

    // println!("hashPrevouts is: {}",hashPrevouts);
    let data1 = Vec::from_hex(hash_prevouts).unwrap();
    let data2 = Sha256::digest(&data1).to_vec();
    let data3 = Sha256::digest(&data2).to_vec();
    let data4 = hex::encode(data3);

    raw_transaction += &data4;

    let mut hash_sequence = String::new();

    for input in data["vin"].as_array().unwrap() {
        let sequence_decimal = input["sequence"].as_u64().unwrap_or_default();
        let sequence_hex = format!("{:08x}", sequence_decimal);
        hash_sequence += &hex_to_little_endian(&sequence_hex);
    }

    // println!("hashSequence is: {}",hashSequence);
    let data1 = Vec::from_hex(hash_sequence).unwrap();
    let data2 = Sha256::digest(&data1).to_vec();
    let data3 = Sha256::digest(&data2).to_vec();
    let data4 = hex::encode(data3);

    raw_transaction += &data4;

    let prev_txid = data["vin"][parameter]["txid"].as_str().unwrap();
    raw_transaction += &hex_to_little_endian(prev_txid);

    let prev_index = format!("{:08x}", data["vin"][parameter]["vout"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&prev_index);

    raw_transaction += "1976a914";
    raw_transaction += data["vin"][parameter]["prevout"]["scriptpubkey_asm"].as_str().unwrap().split_whitespace().nth(2).unwrap();
    raw_transaction += "88ac";

    let value = format!("{:016x}", (data["vin"][parameter]["prevout"]["value"].as_f64().unwrap()) as u64);
    raw_transaction += &hex_to_little_endian(&value);

    let sequence_decimal = data["vin"][parameter]["sequence"].as_u64().unwrap_or_default();
    let sequence_hex = format!("{:08x}", sequence_decimal);
    raw_transaction += &hex_to_little_endian(&sequence_hex);

    let mut hash_outputs = String::new();

    for output in data["vout"].as_array().unwrap() {
        let value = format!("{:016x}", (output["value"].as_f64().unwrap()) as u64);
        hash_outputs += &hex_to_little_endian(&value);

        let temp = output["scriptpubkey"].as_str().unwrap().len() / 2;

        hash_outputs += &hex_to_little_endian(&format!("{:02x}", temp));
        hash_outputs += output["scriptpubkey"].as_str().unwrap();

    }

    let data1 = Vec::from_hex(hash_outputs).unwrap();
    let data2 = Sha256::digest(&data1).to_vec();
    let data3 = Sha256::digest(&data2).to_vec();
    let data4 = hex::encode(data3);

    raw_transaction += &data4;

    let locktime = format!("{:08x}", data["locktime"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&locktime);

    let sighash_all = "01000000";
    raw_transaction += sighash_all;

    // println!("raw_transaction is: {}",raw_transaction);

    let data1 = Vec::from_hex(raw_transaction).unwrap();
    let data2 = Sha256::digest(&data1).to_vec();
    let hash_hex1 = Sha256::digest(&data2).to_vec();

    let hex = hex::encode(hash_hex1);
    hex
}
