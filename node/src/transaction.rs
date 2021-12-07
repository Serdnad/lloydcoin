use std::convert::TryInto;
use crate::LC as LC;
use serde_json::{json};

use sha2::{Sha256, Digest};

use rsa::{RsaPublicKey, pkcs8, hash, padding, PublicKey};
use rsa::Hash::SHA2_256;

fn hash_data(data: LC::SignedTransactionData) -> Vec<u8> {
    let data_without_sig = json!(LC::TransactionData {
        sender_key: data.sender_key,
        amount: data.amount,
        receiver_key: data.receiver_key
    }).to_string();

    let mut hasher = Sha256::new();
    hasher.update(data_without_sig.as_bytes());
    hasher.finalize().to_vec()
}


use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}, ecdsa};

use k256::{
    ecdsa::{SigningKey, signature::Signer},
    SecretKey,
};
use k256::ecdsa::DerSignature;
use k256::ecdsa::recoverable::Id;
use k256::ecdsa::signature::Signature;
use crate::LC::{SignedTransactionData, TransactionData};

pub fn transaction_request(transaction: Option<String>) -> Result<SignedTransactionData, &str> {
    let data_str = transaction.unwrap();
    let data: LC::SignedTransactionData = serde_json::from_str(&data_str).unwrap();

    println!("{}", data_str);

    let t = TransactionData {
        receiver_key: data.receiver_key.clone(),
        sender_key: data.sender_key.clone(),
        amount: data.amount.clone(),
    };

    validate_transaction(t, &data.signature).unwrap();

    Ok(data)
}


/// Validate a transaction.
///
/// A transaction is valid if the following hold:
/// - (TODO) The transaction is dated in the past.
/// - The private key used to sign the transaction (signature) matches the public key in sender_key.
///
/// Note: `signature` must be a ECDSA/secp256k1 signature (ASN.1 DER encoded) in hex format.
fn validate_transaction(transaction: TransactionData, signature: &str) -> Result<(), &str> {
    let signature_bytes = hex::decode(signature).unwrap();
    let der_signature = k256::ecdsa::DerSignature::from_bytes(signature_bytes.as_slice()).unwrap();
    let signature = k256::ecdsa::Signature::from_der(der_signature.as_bytes()).unwrap();

    let sender_key_bytes = hex::decode(&transaction.sender_key).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(sender_key_bytes.as_slice()).unwrap();

    let mut digest = sha3::Sha3_256::new();
    digest.update(json!(transaction).to_string().as_bytes());

    let recoverable_signature = k256::ecdsa::recoverable::Signature::from_digest_trial_recovery(&verifying_key, digest.clone(), &signature).unwrap();
    let recovered_key = recoverable_signature.recover_verify_key_from_digest(digest.clone()).unwrap();

    if sender_key_bytes.as_slice() == recovered_key.to_bytes().as_slice() {
        Ok(())
    } else {
        Err("sender_key does not correspond to signature")
    }
}

// TODO: unit test ^