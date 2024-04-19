extern crate secp256k1;
extern crate hex;

use secp256k1::{Secp256k1, Message, PublicKey, Signature, Error};
use hex::{decode, FromHex};

use std::fs::File;
use std::io::Read;
use sha2::{Digest, Sha256};
use std::str;
use std::fs;

use std::time::Instant;

mod create_transaction;
mod create_txid;
mod merkle_root;

use std::fs::OpenOptions;
use std::io::Write;

use crate::create_txid::coinbase;
use crate::create_txid::header;



fn append_string_to_file(file_path: &str, content: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    writeln!(file, "{}", content)?;

    Ok(())
}

fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}


fn main() {
    let start = Instant::now();

    // let folder_path = "../p2pkh/";
    // let mut invalid=0;
    // let mut count = 0;
    // for entry in fs::read_dir(folder_path).unwrap() {
    //     if let Ok(entry) = entry {
    //         let file_path = entry.path();
    //         // let file_name = entry.file_name().into_string().unwrap();
    //         println!("{}", file_path.display());
    //         let mut f = File::open(file_path).unwrap();
    //         // let mut f = File::open("../mempool/8680a81d506c73841c10013cbc89bebf5d4cae96c3e1bbdb66540c3df58864ff.json").unwrap();
    //         let mut data = String::new();
    //         f.read_to_string(&mut data).unwrap();
    //         let data: serde_json::Value = serde_json::from_str(&data).unwrap();

    //         let input_count = data["vin"].as_array().unwrap().len();
    //         // let hex= create_transaction_p2wpkh(data.clone(),0);

    //         let mut flag = false;

    //         let type_of_transaction = data["vin"][0]["prevout"]["scriptpubkey_type"].as_str().unwrap().to_string();

    //         for i in 0..input_count {
    //             let script_sigtype = data["vin"][i]["prevout"]["scriptpubkey_type"].as_str().unwrap();

    //             if script_sigtype != type_of_transaction {
    //                 flag = true;
    //                 break;
    //             }

    //             let hex = if script_sigtype == "p2pkh" {
    //                 create_transaction::p2pkh::create_transaction_p2pkh(data.clone(), i)
    //             } else if script_sigtype == "v0_p2wpkh" {
    //                 create_transaction::p2wpkh::create_transaction_p2wpkh(data.clone(), i)
    //             } else {
    //                 flag = true;
    //                 break;
    //             };
    //             let hash_hex: &str = hex.as_str();

    //             let secp = Secp256k1::new();
                
    //             // Decode the signature, public key, and hash

    //             let signature: String = if script_sigtype == "p2pkh" {
    //                 data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(1).unwrap().to_string()
    //             } else if script_sigtype == "v0_p2wpkh" {
    //                 data["vin"][i]["witness"][0].as_str().unwrap().to_string()
    //             } else {
    //                 flag = true;
    //                 break;
    //             };

    //             // IF P2PKH SIGNATURE IS HERE
    //             let signature = &signature[..signature.len() - 2];
    //             let signature_bytes = decode(signature).expect("Failed to decode signature hex");

    //             // IF P2PKH PUBKEY IS HERE
    //             let pub_key: String = if script_sigtype == "p2pkh" {
    //                 data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap().to_string()
    //             } else if script_sigtype == "v0_p2wpkh" {
    //                 data["vin"][i]["witness"][1].as_str().unwrap().to_string()
    //             } else {
    //                 flag = true;
    //                 break;
    //             };

    //             // let pub_key = data["vin"][i]["scriptsig_asm"].as_str().unwrap().split_whitespace().nth(3).unwrap();

    //             // let pub_key = data["vin"][i]["witness"][1].as_str().unwrap();
    //             let pubkey_bytes = decode(pub_key).expect("Failed to decode pubkey hex");
    //             let pubkey = PublicKey::from_slice(&pubkey_bytes).expect("Invalid public key");

    //             let hash_bytes = decode(hash_hex).expect("Failed to decode hash hex");
    //             let message = Message::from_slice(&hash_bytes).expect("Invalid message");

    //             // Create a signature object
    //             let signature = Signature::from_der(&signature_bytes).expect("Invalid signature");

    //             // Verify the signature
    //             match secp.verify(&message, &signature, &pubkey) {
    //                 Ok(_) => {
    //                     println!("Signature is valid!");
    //                     continue;
    //                 },
    //                 Err(Error::IncorrectSignature) =>
    //                 { 
    //                     println!("Signature is invalid!");
    //                     flag = true;
    //                     break;
    //                 },
    //                 _ => println!("Failed to verify signature!"),
    //             }
    //         }
            
    //         if flag == false {
    //             // println!("Transaction is valid");
    //             let type_of_transaction = data["vin"][0]["prevout"]["scriptpubkey_type"].as_str().unwrap().to_string();
    //             let hex = if type_of_transaction == "p2pkh" {
    //                 create_txid::p2pkh::create_transaction_p2pkh_final(data.clone())
    //             } else if type_of_transaction == "v0_p2wpkh" {
    //                 create_txid::p2wpkh::create_transaction_p2wpkh_final(data.clone())
    //             } else {
    //                 continue;
    //             };
    //             // let hex = create_transaction_p2pkh_final(data.clone());
    //             let hash_hex: &str = hex.as_str();
    //             let data1 = Vec::from_hex(hash_hex).unwrap();
    //             let data2 = Sha256::digest(&data1).to_vec();
    //             let hash_hex1 = Sha256::digest(&data2).to_vec();
    //             let hex = hex::encode(hash_hex1);
    //             println!("Hash of the block is: {}",hex);
    //             let little_endian = hex_to_little_endian(&hex);
    //             if type_of_transaction == "p2pkh" {
    //                 println!("Type of the block is: P2PKH");
    //             }
    //             else if type_of_transaction == "v0_p2wpkh"{
    //                 println!("Type of the block is: P2WPKH");
    //             }
    //             println!("little of the block is: {}",little_endian);

    //             let wxid = if type_of_transaction == "p2pkh" {
    //                 create_txid::p2pkh::create_transaction_p2pkh_final(data.clone())
    //             } else if type_of_transaction == "v0_p2wpkh" {
    //                 create_txid::p2wpkh::create_transaction_p2wpkh_final(data.clone())
    //                 continue;
    //             } else {
    //                 continue;
    //             };

    //             let hash_hex: &str = wxid.as_str();
    //             let data1 = Vec::from_hex(hash_hex).unwrap();
    //             let data2 = Sha256::digest(&data1).to_vec();
    //             let hash_hex1 = Sha256::digest(&data2).to_vec();
    //             let hex = hex::encode(hash_hex1);
    //             let little_endian = hex_to_little_endian(&hex);
    //             // let mut f = File::create("../block/".to_string() + &hex + ".json").unwrap();
    //             let file_path = "../code_p2pkh.txt";
    //             let content_to_append = little_endian.as_str();
    //             if let Err(err) = append_string_to_file(file_path, content_to_append) {
    //                 eprintln!("Error appending to file: {}", err);
    //             }



    //             count = count + 1;
    //         }
    //         else {
    //             // println!("Transaction is invalid");
    //             invalid = invalid + 1;
    //         }
            
    //     }
    // }   
    // println!("Total number of valid transactions are: {}",count); 
    // println!("Total number of invalid transactions are: {}",invalid);

    let file_path = "../block.txt";

    let file_path = "../code_p2pkh.txt";

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    
    let txids: Vec<String> = contents.lines().map(String::from).collect();

    let mut txids: Vec<String> = txids
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>().chunks(2).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().iter().rev().map(|s| s.to_string()).collect::<String>())
        .collect();

    let result = merkle_root::merkleroot(&mut txids);


    // Convert the result back to natural byte order
    let result_bytes = result.chars().collect::<String>();
    let result_bytes= hex_to_little_endian(&result_bytes);
    println!("tmp: {}", result_bytes); // Output: f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766

    println!("coinbase transaction is: {} ", coinbase::coinbase(result_bytes.clone()));
    let file_path = "../code.txt";
    let file_path = "../code_p2pkh.txt";

    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    
    let txids: Vec<String> = contents.lines().map(String::from).collect();

    let mut txids: Vec<String> = txids
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>().chunks(2).map(|c| c.iter().collect::<String>()).collect::<Vec<String>>().iter().rev().map(|s| s.to_string()).collect::<String>())
        .collect();

    let result = merkle_root::merkleroot(&mut txids);


    // Convert the result back to natural byte order
    let result_bytes = result.chars().collect::<String>();
    let result_bytes= hex_to_little_endian(&result_bytes);
    println!("the merkle root is: {}", result_bytes);
    let header = create_txid::header::header(result_bytes.clone());
    // println!("header is :{}", header);

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
