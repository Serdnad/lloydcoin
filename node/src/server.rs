use std::borrow::Borrow;
use std::hash::Hash;

use serde_json::{json, Value};
use url::form_urlencoded::parse;

use crate::blockchain::Block;
use crate::LC as LC;
use crate::LC::MessageType;
use crate::LC::MessageType::Response;
use crate::node as node;
use crate::transaction as transaction;
use crate::transaction::SignedTransaction;

mod server;
mod handlers;

pub struct Server {
    pub socket: ws::Sender,
    pub node: node::Node,
}

impl Server {
    pub fn handle_data_received(&mut self, msg: ws::Message) {
        // let response: Result<String, &str> = match msg {
        //     ws::Message::Text(s) => self.handle_message(s),
        //     _ => Ok(String::from("huh"))
        // };

        if !msg.is_text() {
            return;
        }

        let msg = msg.into_text().unwrap();
        println!("{}", msg);

        let socket = &self.socket.clone();

        let res = self.handle_message(msg);

        if res.is_ok() {
            socket.send(res.unwrap());
        }


        // if response.is_ok() {
        //     self.socket.send(response.unwrap());
        // }
    }

    /// Determine whether a message received by this node is a new request, or a response to a
    /// request made by this node, and route it accordingly.
    fn handle_message(&mut self, msg: String) -> Result<String, String> {
        let parsed: Result<LC::Message, _> = serde_json::from_str(&msg);
        if parsed.is_err() {
            println!("ERROR PARSING JSON: {}", parsed.unwrap_err());
            return Err(String::from("ERROR"));
        }

        let parsed = parsed.unwrap();
        match parsed.typ {
            LC::MessageType::Request => self.handle_request(parsed),
            LC::MessageType::Response => self.handle_response(parsed),
            //TODO: ping / pong
            _ => Ok(String::from("ok"))
        }
    }

    // TODO
    // pub fn handle_ping() {}

    // TODO
    // pub fn handle_pong() {}

    /// Route a received request to the intended handler.
    fn handle_request(&mut self, request: LC::Message) -> Result<String, String> {
        if request.action.is_none() {
            return Err(String::from("action is missing"));
        }

        match request.action.as_ref().unwrap().as_ref() {
            "get_nodes" => Ok(self.handle_get_nodes_request()),
            "get_block" => handlers::get_block(&self.node, &request),
            "transaction" => {
                let tx: SignedTransaction = serde_json::from_str(&request.data.unwrap()).unwrap();

                println!("Move {} from {} to {}", tx.data.amount, tx.data.sender_key, tx.data.receiver_key);

                let block = Block {
                    tx,
                    prev_hash: self.node.chain.back().unwrap().clone(),
                };


                let block_hash = block.hash();
                self.node.blocks.insert(block_hash.clone(), block);
                self.node.chain.push_back(block_hash);

                println!("{:?}", self.node.chain);
                println!("{:?}", self.node.blocks);
                // self.node.blocks.entry(block_hash).insert(block);
                // self.node.chain.push_back(block_hash);
                // let tx = transaction::transaction_request(request.data).unwrap();

                Ok((String::from("dope dope")))
            }
            _ => Ok(String::from("unsupported request")),
        }
    }

    /// Handle a response to a request previously sent out by this node.
    /// TODO: A much better way (i think) to do this would be to store an ID for any request made,
    ///     with relevant data, and then complete them. that'll also make the ping/pong process
    ///     easier I'd think, as far as detecting timeouts.
    fn handle_response(&mut self, response: LC::Message) -> Result<String, String> {
        if response.action.is_none() {
            return Err(String::from("action is missing"));
        }

        match response.action.unwrap().as_ref() {
            "get_nodes" => {
                self.handle_get_nodes_response(response.data);
                Ok(String::from("{}"))
            }
            _ => Ok(String::from("unsupported action"))
        }
    }
}
