use std::convert::TryInto;

use k256::{ecdsa::{signature::Verifier, VerifyingKey}, ecdsa, EncodedPoint};
use k256::{
    ecdsa::{signature::Signer, SigningKey},
    SecretKey,
};
use k256::ecdsa::DerSignature;
use k256::ecdsa::recoverable::Id;
use k256::ecdsa::signature::Signature;
use rsa::{hash, padding, pkcs8, PublicKey, RsaPublicKey};
use rsa::Hash::SHA2_256;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use sha2::{Digest, Sha256};

/// A signed LC transaction, the most atomic unit included in the blockchain.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransaction {
    pub data: TransactionData,
    pub signature: String,
}

/// The data describing a transfer of LloydCoin.
///
/// TransactionData is only useful with a signature signed by the sender. See [SignedTransaction].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionData {
    pub sender_key: String,
    pub amount: u64,
    pub receiver_key: String,
}

// TODO: not 100% sure how this fits in, or if this is where this should go.
pub fn transaction_request(transaction: Option<String>) -> Result<SignedTransaction, &'static str> {
    let data_str = transaction.unwrap();
    let tx: SignedTransaction = serde_json::from_str(&data_str).unwrap();

    println!("{}", data_str);

    let t = TransactionData {
        receiver_key: tx.data.receiver_key.clone(),
        sender_key: tx.data.sender_key.clone(),
        amount: tx.data.amount.clone(),
    };

    validate_transaction(&tx).unwrap();

    Ok(tx)
}

/// Validate a transaction.
///
/// A transaction is valid if the following hold:
/// - (TODO) The transaction is dated in the past.
/// - The private key used to sign the transaction (signature) matches the public key in sender_key.
///
/// Note: `signature` must be a ECDSA/secp256k1 signature (ASN.1 DER encoded) in hex format.
pub fn validate_transaction(transaction: &SignedTransaction) -> Result<(), &str> {
    let signature_bytes = hex::decode(&transaction.signature).unwrap();
    let der_signature = k256::ecdsa::DerSignature::from_bytes(signature_bytes.as_slice()).unwrap();
    let signature = k256::ecdsa::Signature::from_der(der_signature.as_bytes()).unwrap();

    let sender_key_bytes = hex::decode(&transaction.data.sender_key).unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(sender_key_bytes.as_slice()).unwrap();

    let mut digest = sha3::Sha3_256::new();
    digest.update(json!(&transaction.data).to_string().as_bytes());

    let recoverable_signature = k256::ecdsa::recoverable::Signature::from_digest_trial_recovery(&verifying_key, digest.clone(), &signature).unwrap();
    let recovered_key = recoverable_signature.recover_verify_key_from_digest(digest.clone()).unwrap();

    if sender_key_bytes.as_slice() == recovered_key.to_bytes().as_slice() {
        Ok(())
    } else {
        Err("sender_key does not correspond to signature")
    }
}

// TODO: unit test ^
