use serde_json::{Value, json};

extern crate rand;
extern crate ed25519_dalek;

use rsa::{RsaPublicKey, RsaPrivateKey};
use rsa::{pkcs8, hash, padding};
use rsa::PublicKey;

use sha2::{Sha256, Digest};

use crate::node as node;
use crate::LC as LC;

pub struct Server {
    pub socket: ws::Sender,
    pub node: node::Node
}

impl Server {
    pub fn request_nodes(&self) {
        let request = json!(LC::Message {
                        typ: LC::MessageType::Request,
                        action: Some("get_nodes".to_string()),
                        data: None,
                    });
        self.socket.send(request.to_string());
    }

    pub fn handle_data_received(&mut self, msg: ws::Message) {
        let response: Option<String> = match msg {
            ws::Message::Text(s) => self.handle_message(s),
            _ => None
        };

        if response.is_some() {
            self.socket.send(response.unwrap());
        }
    }


    pub fn get_nodes_request(&mut self) -> Option<String> {
        let connections = self.node.get_connections();
        let resp_data = json!(connections.into_iter().collect::<Vec<String>>());

        let response = json!(LC::Message {
            typ: LC::MessageType::Response,
            action: Some("get_nodes".to_string()),
            data: Some(resp_data.to_string())
        });
        
        Some(response.to_string())
    }

    pub fn hash_data(&self, data: LC::SignedTransactionData) -> Vec<u8> {
        let data_without_sig = json!(LC::TransactionData {
            sender_id: data.sender_id,
            amount: data.amount,
            receiver_id: data.receiver_id
        }).to_string();

        let mut hasher = Sha256::new();
        hasher.update(data_without_sig.as_bytes());
        hasher.finalize().to_vec()
    }

    pub fn transaction_request(&mut self, transaction: Option<String>) -> Option<String> {
        let data_str = transaction.unwrap();
        let data: LC::SignedTransactionData = serde_json::from_str(&data_str).unwrap();

        let sender_pem = &data.sender_id;
        let sender_public_key: RsaPublicKey = pkcs8::FromPublicKey::from_public_key_pem(&sender_pem).unwrap();

        let hashed_data = self.hash_data(data.clone());

        let padding = padding::PaddingScheme::new_pkcs1v15_sign(Some(hash::Hash::SHA2_256));
        let signature = data.signature.as_bytes();
        let result = sender_public_key.verify(padding, &hashed_data, signature);

        println!("{:?}", result);

        None
    }

    pub fn handle_request(&mut self, request: LC::Message) -> Option<String> {
        match request.action {
            Some(action) => {
                match &*action {
                    "get_nodes" => self.get_nodes_request(),
                    "transaction" => self.transaction_request(request.data),
                    _ => None
                }
            },
            _ => None
        }
    }

    pub fn get_nodes_response(&mut self, data: Option<String>) {
        if data.is_some() {
            let str_vec = data.unwrap();
            println!("{:?}", str_vec);

            let ips: Vec<String> = serde_json::from_str(&str_vec).unwrap();
            self.connect_to_new_ips(ips);
        }
    }

    pub fn handle_response(&mut self, response: LC::Message) -> Option<String> {
        match response.action {
            Some(action) => {
                match &*action {
                    "get_nodes" => {
                        self.get_nodes_response(response.data);
                        None
                    },
                    _ => None
                }
            },
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

    pub fn connect_to_new_ips(&mut self, new_ips: Vec<String>) {
        for ip in new_ips {
            if !self.node.contains_ip(&ip) {
                let url = "ws://".to_owned()+&ip+":9001";
                crate::network::connect_to_ip(url, self.node.clone());
            }
        }
    }
}
