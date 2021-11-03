type PublicKey = String;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub typ: MessageType,
    pub action: Option<String>,
    pub data: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransactionData {
    pub sender_id: PublicKey,
    pub amount: u32,
    pub receiver_id: PublicKey,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransactionData {
    pub sender_id: PublicKey,
    pub amount: u32,
    pub receiver_id: PublicKey,
    pub signature: String,
}
