use sha2::{Digest, Sha256};

fn hash256(hex: &str) -> String {
    let binary = hex::decode(hex).unwrap();
    let hash1 = Sha256::digest(&binary);
    let hash2 = Sha256::digest(&hash1);
    let result = hex::encode(hash2);
    result
}

pub fn merkleroot(txids: &mut Vec<String>) -> String {
    if txids.len() == 1 {
        return txids[0].clone();
    }

    let mut result: Vec<String> = Vec::new();

    for chunk in txids.chunks_mut(2) {
        let concat = if let Some(two) = chunk.get(1) {
            chunk[0].clone() + two
        } else {
            chunk[0].clone() + &chunk[0]
        };

        result.push(hash256(&concat));
    }

    merkleroot(&mut result)
}
