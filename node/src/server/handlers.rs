use serde_json::json;

use crate::blockchain::Block;
use crate::LC::{Message, MessageType};
use crate::{LC, Node};
use crate::server::Server;

pub fn get_block<'a>(node: &Node, request: &Message) -> Result<String, String> {
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