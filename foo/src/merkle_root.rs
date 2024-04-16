// use sha2::{Digest, Sha256};

// fn hash256(hex: &str) -> String {
//     let binary = hex::decode(hex).unwrap();
//     let hash1 = Sha256::digest(&binary);
//     let hash2 = Sha256::digest(&hash1);
//     let result = hex::encode(hash2);
//     result
// }

// pub fn merkleroot(txids: &mut Vec<String>) -> String {
//     if txids.len() == 1 {
//         return txids[0].clone();
//     }

//     let mut result: Vec<String> = Vec::new();

//     for chunk in txids.chunks_mut(2) {
//         let concat = if let Some(two) = chunk.get(1) {
//             chunk[0].clone() + two
//         } else {
//             chunk[0].clone() + &chunk[0]
//         };

//         result.push(hash256(&concat));
//     }

//     merkleroot(&mut result)
// }

// fn main() {
//     let mut txids = vec![
//         "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87",
//         "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4",
//         "6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4",
//         "e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d",
//     ];

//     // Convert hex strings to byte order
//     for txid in &mut txids {
//         *txid = txid.chars().rev().collect();
//     }

//     let result = merkleroot(&mut txids);

//     // Convert the result back to natural byte order
//     let result_bytes = result.chars().rev().collect::<String>();

//     println!("{}", result_bytes); // Output: f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766
// }
