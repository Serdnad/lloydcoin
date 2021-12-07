use serde_json::{Value, json};

use crate::node as node;
use crate::LC as LC;
use crate::transaction as transaction;

pub struct Server {
    pub socket: ws::Sender,
    pub node: node::Node,
}

impl Server {
    pub fn handle_data_received(&mut self, msg: ws::Message) {
        let response: Option<String> = match msg {
            ws::Message::Text(s) => self.handle_message(s),
            _ => None
        };

        if response.is_some() {
            self.socket.send(response.unwrap());
        }
    }

    pub fn handle_request(&mut self, request: LC::Message) -> Option<String> {
        match request.action {
            Some(action) => {
                match &*action {
                    "get_nodes" => self.get_nodes_request(),
                    "transaction" => {
                        let Ok(tx) = transaction::transaction_request(request.data);

                        None
                    }
                    _ => None,
                }
            }
            _ => None
        }
    }

    pub fn handle_response(&mut self, response: LC::Message) -> Option<String> {
        match response.action {
            Some(action) => {
                match &*action {
                    "get_nodes" => {
                        self.get_nodes_response(response.data);
                        None
                    }
                    _ => None
                }
            }
            _ => None
        }
    }

    pub fn handle_message(&mut self, msg: String) -> Option<String> {
        let parsed: LC::Message = serde_json::from_str(&msg).unwrap();

        match parsed.typ {
            LC::MessageType::Request => self.handle_request(parsed),
            LC::MessageType::Response => self.handle_response(parsed)
        }
    }
}
