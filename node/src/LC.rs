use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Request,
    Response,
    CreatedBlock,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub typ: MessageType,
    pub action: Option<String>,
    pub data: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct SignedTransaction {
//     pub data: TransactionData,
//     pub signature: String,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// pub struct TransactionData {
//     pub sender_key: String,
//     pub amount: u32,
//     pub receiver_key: String,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignedTransactionData {
    pub sender_key: String,
    pub amount: u32,
    pub receiver_key: String,
    pub signature: String,
}
