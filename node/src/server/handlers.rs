use serde_json::json;

use crate::blockchain::Block;
use crate::LC::{Message, MessageType};
use crate::{LC, Node};
use crate::server::Server;
use crate::transaction::{SignedTransaction, validate_transaction};

/// Handle a request for an account's balance.
pub fn get_balance(node: &Node, request: &Message) -> Result<String, String> {
    match &request.data {
        None => Err(String::from("no account supplied")),
        Some(account_key) => {
            // let balance = node.balance_manager.get_balance(account_key);
            Ok(json!(Message{
                typ: MessageType::Response,
                action: Some(String::from("get_balance")),
                data: Some(String::from("TODO"))
            }).to_string())
        }
    }
}

/// Handle a request for a block.
pub fn get_block(node: &Node, request: &Message) -> Result<String, String> {
    match &request.data {
        None => Err(String::from("no block hash supplied")),
        Some(block_hash) => {
            match node.blocks.get(block_hash) {
                None => Err(format!("no block found with hash {}", block_hash)),
                Some(block) => Ok(
                    json!(Message{
                        typ: MessageType::Response,
                        action: Some(String::from("get_block")),
                        data: Some(json!(block).to_string()),
                    }).to_string()
                )
            }
        }
    }
}

/// Validate a transaction and submit it to the network if valid.
pub fn add_transaction(node: &mut Node, data: String) -> Result<String, String> {
    let tx: SignedTransaction = serde_json::from_str(&data).unwrap();

    println!("Move {} from {} to {}", tx.data.amount, tx.data.sender_key, tx.data.receiver_key);

    if let Err(a) = validate_transaction(&tx) {
        return Err(a.to_string());
    }

    // if let Err(err) = node.balance_manager.lock().unwrap()
    // if let Err(a) = node. &node) {
    //     return Err(a.to_string());
    // }

    let prev_hash = node.chain.back().unwrap().clone();
    let block = Block { tx, prev_hash };

    let block_hash = block.hash();
    node.blocks.insert(block_hash.clone(), block);
    node.chain.push_back(block_hash);

    println!("{:?}", node.chain);
    println!("{:?}", node.blocks);
    // self.node.blocks.entry(block_hash).insert(block);
    // self.node.chain.push_back(block_hash);
    // let tx = transaction::transaction_request(request.data).unwrap();

    Ok((String::from("dope dope")))
}
