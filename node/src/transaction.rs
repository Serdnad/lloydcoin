use crate::LC as LC;
use serde_json::{json};

use sha2::{Sha256, Digest};

use rsa::{RsaPublicKey, pkcs8, hash, padding, PublicKey};

fn hash_data(data: LC::SignedTransactionData) -> Vec<u8> {
    let data_without_sig = json!(LC::TransactionData {
        sender_id: data.sender_id,
        amount: data.amount,
        receiver_id: data.receiver_id
    }).to_string();

    let mut hasher = Sha256::new();
    hasher.update(data_without_sig.as_bytes());
    hasher.finalize().to_vec()
}

pub fn transaction_request(transaction: Option<String>) -> Option<String> {
    let data_str = transaction.unwrap();
    let data: LC::SignedTransactionData = serde_json::from_str(&data_str).unwrap();

    let sender_pem = &data.sender_id;
    let sender_public_key: RsaPublicKey = pkcs8::FromPublicKey::from_public_key_pem(&sender_pem).unwrap();

    let hashed_data = hash_data(data.clone());

    let padding = padding::PaddingScheme::new_pkcs1v15_sign(Some(hash::Hash::SHA2_256));
    let signature = data.signature.as_bytes();
    let result = sender_public_key.verify(padding, &hashed_data, signature);

    println!("{:?}", result);

    None
}
