
extern crate secp256k1;
extern crate hex;
use std::str;


fn hex_to_little_endian(hex_number: &str) -> String {
    let hex_bytes = hex::decode(hex_number).unwrap();
    let mut little_endian_bytes = hex_bytes.clone();
    little_endian_bytes.reverse();
    hex::encode(little_endian_bytes)
}

pub fn create_transaction_p2pkh_final(data: serde_json::Value) -> String {
    let mut raw_transaction = String::new();

    let version = format!("{:08x}", data["version"].as_u64().unwrap());
    raw_transaction += &hex_to_little_endian(&version);

    let input_count = format!("{:02x}", data["vin"].as_array().unwrap().len());


    let input_count: u8 =  data["vin"].as_array().unwrap().len() as u8;
    let input_count_bytes = hex::encode(input_count.to_le_bytes());

    raw_transaction += &hex_to_little_endian(&input_count_bytes);

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
    
    let output_count: u8 =  data["vout"].as_array().unwrap().len() as u8;
    let output_count_bytes = hex::encode(output_count.to_le_bytes());
    raw_transaction += &hex_to_little_endian(&output_count_bytes);

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


