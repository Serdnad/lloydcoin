use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha3::{Digest, Sha3_256};

use crate::transaction::SignedTransaction;

type Hash = String;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    pub prev_hash: Hash,
    pub tx: SignedTransaction,
}

impl Block {
    pub fn hash(&self) -> String {
        let mut digest = Sha3_256::new();

        digest.update(json!(self).to_string().as_bytes());
        hex::encode(digest.finalize().as_slice())
    }
}
