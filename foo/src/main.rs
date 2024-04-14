extern crate secp256k1;
extern crate hex;

use secp256k1::{Secp256k1, Message, PublicKey, Signature, Error};
use hex::{decode, FromHex};

use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use std::str;
use std::fs;

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

fn create_transaction_p2wpkh_final(data: serde_json::Value) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&input_count);

    let mut ind: usize = 0;

    for input in data["vin"].as_array().unwrap() {
        
        // println!("index is {}",ind);
        let prev_txid = input["txid"].as_str().unwrap();
        raw_transaction += &hex_to_little_endian(prev_txid);

        let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
        raw_transaction += &hex_to_little_endian(&prev_index);

        let scriptsig_length = input["scriptsig"].as_str().unwrap().len() / 2;
        let scriptsig_length_hex = format!("{:02x}", scriptsig_length);
        raw_transaction += &hex_to_little_endian(&scriptsig_length_hex);

        let sequence_decimal = data["vin"][ind]["sequence"].as_u64().unwrap_or_default();
        let sequence_hex = format!("{:08x}", sequence_decimal);

        raw_transaction += &hex_to_little_endian(&sequence_hex);
        
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
    raw_transaction
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

fn create_transaction_p2pkh_final(data: serde_json::Value) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());
    raw_transaction += &hex_to_little_endian(&input_count);

    let mut ind: usize = 0;

    for input in data["vin"].as_array().unwrap() {
        
        // println!("index is {}",ind);
        let prev_txid = input["txid"].as_str().unwrap();
        raw_transaction += &hex_to_little_endian(prev_txid);

        let prev_index = format!("{:08x}", input["vout"].as_u64().unwrap());
        raw_transaction += &hex_to_little_endian(&prev_index);

        let scriptsig_length = input["scriptsig"].as_str().unwrap().len() / 2;
        let scriptsig_length_hex = format!("{:02x}", scriptsig_length);
        raw_transaction += &hex_to_little_endian(&scriptsig_length_hex);

        let scriptsig = input["scriptsig"].as_str().unwrap();
        raw_transaction += scriptsig;

        let sequence_decimal = data["vin"][ind]["sequence"].as_u64().unwrap_or_default();
        let sequence_hex = format!("{:08x}", sequence_decimal);

        raw_transaction += &hex_to_little_endian(&sequence_hex);
        
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
    raw_transaction
}



fn main() {
    let folder_path = "../mempool";
    let mut invalid=0;
    let mut count = 0;
    for entry in fs::read_dir(folder_path).unwrap() {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            // let file_name = entry.file_name().into_string().unwrap();
            println!("{}", file_path.display());
            let mut f = File::open(file_path).unwrap();
            // let mut f = File::open("../mempool/8680a81d506c73841c10013cbc89bebf5d4cae96c3e1bbdb66540c3df58864ff.json").unwrap();
            let mut data = String::new();
            f.read_to_string(&mut data).unwrap();
            let data: serde_json::Value = serde_json::from_str(&data).unwrap();

            let input_count = data["vin"].as_array().unwrap().len();
            // let hex= create_transaction_p2wpkh(data.clone(),0);

            let mut flag = false;

            let type_of_transaction = data["vin"][0]["prevout"]["scriptpubkey_type"].as_str().unwrap().to_string();

            for i in 0..input_count {
                let script_sigtype = data["vin"][i]["prevout"]["scriptpubkey_type"].as_str().unwrap();

                if script_sigtype != type_of_transaction {
                    flag = true;
                    break;
                }

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
                    Ok(_) => {
                        println!("Signature is valid!");
                        continue;
                    },
                    Err(Error::IncorrectSignature) =>
                    { 
                        println!("Signature is invalid!");
                        flag = true;
                        break;
                    },
                    _ => println!("Failed to verify signature!"),
                }
            }
            
            if flag == false {
                // println!("Transaction is valid");
                let type_of_transaction = data["vin"][0]["prevout"]["scriptpubkey_type"].as_str().unwrap().to_string();
                let hex = if type_of_transaction == "p2pkh" {
                    create_transaction_p2pkh_final(data.clone())
                } else if type_of_transaction == "v0_p2wpkh" {
                    create_transaction_p2wpkh_final(data.clone())
                } else {
                    flag = true;
                    break;
                };
                // let hex = create_transaction_p2pkh_final(data.clone());
                let hash_hex: &str = hex.as_str();
                let data1 = Vec::from_hex(hash_hex).unwrap();
                let data2 = Sha256::digest(&data1).to_vec();
                let hash_hex1 = Sha256::digest(&data2).to_vec();
                let hex = hex::encode(hash_hex1);
                println!("Hash of the block is: {}",hex);
                let little_endian = hex_to_little_endian(&hex);
                if type_of_transaction == "p2pkh" {
                    println!("Type of the block is: P2PKH");
                }
                else if type_of_transaction == "v0_p2wpkh"{
                    println!("Type of the block is: P2WPKH");
                }
                println!("little of the block is: {}",little_endian);

                // let mut f = File::create("../block/".to_string() + &hex + ".json").unwrap();



                count = count + 1;
            }
            else {
                // println!("Transaction is invalid");
                invalid = invalid + 1;
            }
            
        }
    }   
    println!("Total number of valid transactions are: {}",count); 
    println!("Total number of invalid transactions are: {}",invalid);
}
