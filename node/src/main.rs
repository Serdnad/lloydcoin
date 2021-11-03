use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use url::Url;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use crate::LC::MessageType;

extern crate ws;
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};

extern crate rand;
extern crate ed25519_dalek;

use rsa::{RsaPublicKey, RsaPrivateKey};
use rsa::{pkcs8, hash, padding};
use rsa::PublicKey;

use sha2::{Sha256, Digest};

mod node;

mod LC {
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
}

struct Server {
    socket: Sender,
    node: node::Node
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {

        let ip_addr = shake.remote_addr()?.unwrap();
        self.node.add_new_connection(&self.socket, ip_addr);

        self.request_nodes();


        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.handle_data_received(msg);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Client connection closing: {}", reason)
    }
}

impl Server {
    fn request_nodes(&self) {
        let request = json!(LC::Message {
                        typ: LC::MessageType::Request,
                        action: Some("get_nodes".to_string()),
                        data: None,
                    });
        self.socket.send(request.to_string());
    }

    fn handle_data_received(&mut self, msg: Message) {
        let response: Option<String> = match msg {
            Message::Text(s) => self.handle_message(s),
            _ => None
        };

        if response.is_some() {
            self.socket.send(response.unwrap());
        }
    }


    fn get_nodes_request(&mut self) -> Option<String> {
        let connections = self.node.get_connections();
        let resp_data = json!(connections.into_iter().collect::<Vec<String>>());

        let response = json!(LC::Message {
            typ: LC::MessageType::Response,
            action: Some("get_nodes".to_string()),
            data: Some(resp_data.to_string())
        });
        
        Some(response.to_string())
    }

    fn hash_data(&self, data: LC::SignedTransactionData) -> Vec<u8> {
        let data_without_sig = json!(LC::TransactionData {
            sender_id: data.sender_id,
            amount: data.amount,
            receiver_id: data.receiver_id
        }).to_string();

        let mut hasher = Sha256::new();
        hasher.update(data_without_sig.as_bytes());
        hasher.finalize().to_vec()
    }

    fn transaction_request(&mut self, transaction: Option<String>) -> Option<String> {
        let data_str = transaction.unwrap();
        let data: LC::SignedTransactionData = serde_json::from_str(&data_str).unwrap();

        let sender_pem = &data.sender_id;
        let sender_public_key: RsaPublicKey = pkcs8::FromPublicKey::from_public_key_pem(&sender_pem).unwrap();

        let hashed_data = self.hash_data(data.clone());

        let padding = padding::PaddingScheme::new_pkcs1v15_sign(Some(hash::Hash::SHA2_256));
        let signature = data.signature.as_bytes();
        use rsa::PublicKey;
        use rsa::RsaPublicKey;
        let result = sender_public_key.verify(padding, &hashed_data, signature);

        println!("{:?}", result);

        None
    }

    fn handle_request(&mut self, request: LC::Message) -> Option<String> {
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

    fn get_nodes_response(&mut self, data: Option<String>) {
        if data.is_some() {
            let str_vec = data.unwrap();
            println!("{:?}", str_vec);

            let ips: Vec<String> = serde_json::from_str(&str_vec).unwrap();
            self.connect_to_new_ips(ips);
        }
    }

    fn handle_response(&mut self, response: LC::Message) -> Option<String> {
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

    fn handle_message(&mut self, msg: String) -> Option<String> {
        let parsed: LC::Message = serde_json::from_str(&msg).unwrap();

        match parsed.typ {
            MessageType::Request => self.handle_request(parsed),
            MessageType::Response => self.handle_response(parsed)
        }
    }

    fn connect_to_new_ips(&mut self, new_ips: Vec<String>) {
        for ip in new_ips {
            if !self.node.contains_ip(&ip) {
                let url = "ws://".to_owned()+&ip+":9001";
                connect_to_ip(url, self.node.clone());
            }
        }
    }
}



fn run_server(node: node::Node) {

    let localhost = "0.0.0.0:9001";

    spawn(move || listen(localhost, 
        |socket| {
            Server {
                socket: socket,
                node: node.clone()
            }
        }).unwrap()
    );
    println!("Running server!");
}

fn connect_to_ip(ip: String, node: node::Node) {

    spawn(move || {
        connect(ip, |socket| {
            Server {
                socket: socket,
                node: node.clone()
            }
        }).unwrap()
    });
}

fn main() {
    let node: node::Node = Default::default();

    run_server(node.clone());

    let GOD: String = "ws://10.8.57.232:9001".to_string();
    connect_to_ip(GOD, node.clone());

    loop {
    }
}
