use crate::blockchain::block::{validate_proof_of_work, Block};
use crate::transaction::{validate_transaction, SignedTransaction};
use crate::Node;
use crate::LC::{Message, MessageType};
use serde_json::json;

use hex::FromHex;

extern crate rand;
use rand::Rng;

/// Handle a request for an account's balance.
pub fn get_balance(node: &Node, request: &Message) -> Result<Option<String>, String> {
    match &request.data {
        None => Err(String::from("no account supplied")),
        Some(account_key) => {
            // let balance = node.balance_manager.get_balance(account_key);
            Ok(Some(
                json!(Message {
                    typ: MessageType::Response,
                    action: Some(String::from("get_balance")),
                    data: Some(String::from("TODO"))
                })
                .to_string(),
            ))
        }
    }
}

/// Handle a request for a block.
pub fn get_block(node: &Node, request: &Message) -> Result<Option<String>, String> {
    match &request.data {
        None => Err(String::from("no block hash supplied")),
        Some(block_hash) => match node.blocks.get(block_hash) {
            None => Err(format!("no block found with hash {}", block_hash)),
            Some(block) => Ok(Some(
                json!(Message {
                    typ: MessageType::Response,
                    action: Some(String::from("get_block")),
                    data: Some(json!(block).to_string()),
                })
                .to_string(),
            )),
        },
    }
}

pub fn validate_and_add_block(node: &mut Node, data: String) -> Result<Option<String>, String> {
    let block: Block = serde_json::from_str(&data).unwrap();

    if let Err(a) = validate_proof_of_work(&block, node.threshold) {
        return Err(a.to_string());
    }

    if block.prev_hash != node.chain.back().unwrap() {
        return Err("Invalid prev_hash on received block".to_string());
    }

    if let Err(a) = validate_transaction(&block.tx) {
        return Err(a.to_string());
    }

    println!("Valid block received!");

    node.add_block(block);

    Ok(None)
}

/// Validate a transaction and submit it to the network if valid.
pub fn validate_and_mine_transaction(
    node: &mut Node,
    data: String,
) -> Result<Option<String>, String> {
    let tx: SignedTransaction = serde_json::from_str(&data).unwrap();

    println!(
        "Move {} from {} to {}",
        tx.data.amount, tx.data.sender_key, tx.data.receiver_key
    );

    if let Err(a) = validate_transaction(&tx) {
        return Err(a.to_string());
    }

    println!("Valid signature");

    if let Err(a) = node.balance_manager.process_transaction(&tx.data) {
        return Err(a.to_string());
    }

    println!("Valid amount");

    let prev_hash = node.chain.back().unwrap().clone();
    node.worker_tx.send((tx, prev_hash));

    Ok(None)
}
