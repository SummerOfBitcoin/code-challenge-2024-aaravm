
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

pub fn create_transaction_p2pkh(data: serde_json::Value, parameter: usize) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&input_count);

    let mut ind: usize = 0;

    for input in data["vin"].as_array().unwrap() {
        if ind == parameter {
            // println!("index is {}",ind);
            let prev_txid = input["txid"].as_str().unwrap();
            raw_transaction += &hex_to_little_endian(prev_txid);

            let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
            raw_transaction += &hex_to_little_endian(&prev_index);

            let script_pubkey_length = input["prevout"]["scriptpubkey"].as_str().unwrap().len() / 2;
            let script_pubkey_length_hex = format!("{:02x}", script_pubkey_length);
            raw_transaction += &hex_to_little_endian(&script_pubkey_length_hex);

            let script_pubkey = input["prevout"]["scriptpubkey"].as_str().unwrap();
            raw_transaction += script_pubkey;

            let sequence_decimal = data["vin"][parameter]["sequence"].as_u64().unwrap_or_default();
            let sequence_hex = format!("{:08x}", sequence_decimal);

            raw_transaction += &hex_to_little_endian(&sequence_hex);
        }
        else {
            let prev_txid = input["txid"].as_str().unwrap();
            raw_transaction += &hex_to_little_endian(prev_txid);

            let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
            raw_transaction += &hex_to_little_endian(&prev_index);

            raw_transaction += "00";

            let sequence_decimal = data["vin"][parameter]["sequence"].as_u64().unwrap_or_default();
            let sequence_hex = format!("{:08x}", sequence_decimal);

            raw_transaction += &hex_to_little_endian(&sequence_hex);
        }
        ind = ind + 1;
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
