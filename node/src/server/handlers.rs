use crate::blockchain::block::Block;
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

fn do_work(tx: SignedTransaction, prev_hash: String) -> Block {
    let threshold: [u8; 32] = [
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];

    let mut rng = rand::thread_rng();

    loop {
        let nonce: u64 = rng.gen();
        let block = Block {
            tx: tx.clone(),
            prev_hash: prev_hash.clone(),
            nonce,
        };

        let hash = hex::decode(block.hash()).unwrap();

        let mut less_than = true;
        for (i, elem) in hash.iter().enumerate() {
            if elem > &threshold[i] {
                less_than = false;
                break;
            }
        }
        if less_than {
            return block;
        }
    }
}

/// Validate a transaction and submit it to the network if valid.
pub fn add_transaction(node: &mut Node, data: String) -> Result<Option<String>, String> {
    let tx: SignedTransaction = serde_json::from_str(&data).unwrap();

    println!(
        "Move {} from {} to {}",
        tx.data.amount, tx.data.sender_key, tx.data.receiver_key
    );

    if let Err(a) = validate_transaction(&tx) {
        return Err(a.to_string());
    }

    if let Err(a) = node.balance_manager.process_transaction(&tx.data) {
        return Err(a.to_string());
    }

    let prev_hash = node.chain.back().unwrap().clone();
    let block = do_work(tx, prev_hash);

    let block_hash = block.hash();
    node.blocks.insert(block_hash.clone(), block.clone());
    node.chain.push_back(block_hash);

    println!("{:?}", node.chain);
    println!("{:?}", node.blocks);

    node.broadcast(json!(block).to_string());

    Ok(None)
}
