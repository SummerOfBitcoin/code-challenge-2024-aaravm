extern crate secp256k1;
extern crate hex;

use secp256k1::{Secp256k1, Message, PublicKey, Signature, Error};
use hex::{decode, FromHex};

use std::{fs::File, os::linux::raw};
use std::io::Read;
use sha2::{Digest, Sha256};
use std::str;

fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}

fn create_transaction_p2wpkh(data: serde_json::Value, parameter: usize) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let mut hashPrevouts = String::new();

    for input in data["vin"].as_array().unwrap() {
        let prev_txid = input["txid"].as_str().unwrap();
        hashPrevouts += &hex_to_little_endian(prev_txid);

        let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
        hashPrevouts += &hex_to_little_endian(&prev_index);
    }

    // println!("hashPrevouts is: {}",hashPrevouts);
    let data1 = Vec::from_hex(hashPrevouts).unwrap();
    let data2 = Sha256::digest(&data1).to_vec();
    let data3 = Sha256::digest(&data2).to_vec();
    let data4 = hex::encode(data3);

    raw_transaction += &data4;

    let mut hashSequence = String::new();

    for input in data["vin"].as_array().unwrap() {
        let sequence_decimal = input["sequence"].as_u64().unwrap_or_default();
        let sequence_hex = format!("{:08x}", sequence_decimal);
        hashSequence += &hex_to_little_endian(&sequence_hex);
    }

    // println!("hashSequence is: {}",hashSequence);
    let data1 = Vec::from_hex(hashSequence).unwrap();
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

    let mut hashOutputs = String::new();

    for output in data["vout"].as_array().unwrap() {
        let value = format!("{:016x}", (output["value"].as_f64().unwrap()) as u64);
        hashOutputs += &hex_to_little_endian(&value);

        hashOutputs += "1976a914";
        hashOutputs += output["scriptpubkey_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap();
        hashOutputs += "88ac";
    }

    let data1 = Vec::from_hex(hashOutputs).unwrap();
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


fn create_transaction_p2pkh(data: serde_json::Value, parameter: usize) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&input_count);

    let mut ind: usize = 0;

    for input in data["vin"].as_array().unwrap() {
        if ind == parameter {
            println!("index is {}",ind);
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

fn main() {
    let mut f = File::open("../mempool/00a5be9434f4d97613391cdce760293fd142786a00952ed4edfd66dd19c5c23a.json").unwrap();
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let data: serde_json::Value = serde_json::from_str(&data).unwrap();

    let input_count = data["vin"].as_array().unwrap().len();
    // let hex= create_transaction_p2wpkh(data.clone(),0);

    let mut flag = false;

    for i in 0..input_count {
        // If scriptSigtype!="p2pkh", continue
        let script_sigtype = data["vin"][i]["prevout"]["scriptpubkey_type"].as_str().unwrap();
        // println!("scriptSigtype is: {}",scriptSigtype);


        let hex = if script_sigtype == "p2pkh" {
            create_transaction_p2pkh(data.clone(), i)
        } else if script_sigtype == "v0_p2wpkh" {
            create_transaction_p2wpkh(data.clone(), i)
        } else {
            flag = true;
            break;
        };
        let hash_hex: &str = hex.as_str();

        let secp = Secp256k1::new();
        
        // Decode the signature, public key, and hash

        let signature: String = if script_sigtype == "p2pkh" {
            data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(1).unwrap().to_string()
        } else if script_sigtype == "v0_p2wpkh" {
            data["vin"][i]["witness"][0].as_str().unwrap().to_string()
        } else {
            flag = true;
            break;
        };

        // IF P2PKH SIGNATURE IS HERE
        // let signature = data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(1).unwrap();

        // let signature = data["vin"][i]["witness"][0].as_str().unwrap();
        let signature = &signature[..signature.len() - 2];
        let signature_bytes = decode(signature).expect("Failed to decode signature hex");

        // IF P2PKH PUBKEY IS HERE
        let pub_key: String = if script_sigtype == "p2pkh" {
            data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap().to_string()
        } else if script_sigtype == "v0_p2wpkh" {
            data["vin"][i]["witness"][1].as_str().unwrap().to_string()
        } else {
            flag = true;
            break;
        };

        // let pub_key = data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap();

        // let pub_key = data["vin"][i]["witness"][1].as_str().unwrap();
        let pubkey_bytes = decode(pub_key).expect("Failed to decode pubkey hex");
        let pubkey = PublicKey::from_slice(&pubkey_bytes).expect("Invalid public key");

        let hash_bytes = decode(hash_hex).expect("Failed to decode hash hex");
        let message = Message::from_slice(&hash_bytes).expect("Invalid message");

        // Create a signature object
        let signature = Signature::from_der(&signature_bytes).expect("Invalid signature");

        // Verify the signature
        match secp.verify(&message, &signature, &pubkey) {
            Ok(_) => println!("Signature is valid!"),
            Err(Error::IncorrectSignature) => println!("Signature is invalid!"),
            _ => println!("Failed to verify signature!"),
        }
    }
    

    
}
