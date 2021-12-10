use serde_json::json;

use crate::blockchain::Block;
use crate::LC::{Message, MessageType};
use crate::{LC, Node};
use crate::server::Server;
use crate::transaction::SignedTransaction;

pub fn get_block(node: &Node, request: &Message) -> Result<String, String> {
    match &request.data {
        None => Err(String::from("no block hash supplied")),
        Some(block_hash) => {
            match node.blocks.get(block_hash) {
                None => Err(format!("no block found with hash {}", block_hash)),
                Some(block) => Ok(
                    json!(LC::Message{
                        typ: MessageType::Response,
                        action: Some(String::from("get_block")),
                        data: Some(json!(block).to_string()),
                    }).to_string()
                )
            }
        }
    }
}

pub fn add_transaction(node: &mut Node, data: String) -> Result<String, String> {
    let tx: SignedTransaction = serde_json::from_str(&data).unwrap();

    println!("Move {} from {} to {}", tx.data.amount, tx.data.sender_key, tx.data.receiver_key);

    let block = Block {
        tx,
        prev_hash: node.chain.back().unwrap().clone(),
    };


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
