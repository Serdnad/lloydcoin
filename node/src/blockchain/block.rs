use serde::{Deserialize, Serialize};
use serde_json::json;
use sha3::{Digest, Sha3_256};

use crate::transaction::SignedTransaction;

type Hash = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub prev_hash: Hash,
    pub tx: SignedTransaction,
    pub nonce: u64,
}

impl Block {
    pub fn hash(&self) -> String {
        let mut digest = Sha3_256::new();

        digest.update(json!(self).to_string().as_bytes());
        hex::encode(digest.finalize().as_slice())
    }
}

impl Clone for Block {
    fn clone(&self) -> Self {
        Block {
            prev_hash: self.prev_hash.clone(),
            tx: self.tx.clone(),
            nonce: self.nonce.clone(),
        }
    }
}
