use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use url::Url;
use std::collections::HashMap;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use crate::LC::MessageType;

use std::sync::{Mutex, Arc};

extern crate ws;
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};

mod LC {
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
}

type Connections = Arc<Mutex<HashMap<String, Sender>>>;

struct Node {
    connections: Connections
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            connections: Arc::clone(&self.connections)
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            connections: Arc::new(Mutex::new(HashMap::new()))
        }
    }
}

impl Node {
    fn add_new_connection(&mut self, socket: &Sender, ip_addr: String) {
        let mut map = self.connections.lock().unwrap();
        map.insert(ip_addr, socket.clone());
    }

    fn get_connections(&mut self) -> Vec<String> {
        let mut map = self.connections.lock().unwrap();
        map.keys().cloned().collect()
    }
}

struct Server {
    socket: Sender,
    node: Node
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {

        let ip_addr = shake.remote_addr()?.unwrap();
        self.node.add_new_connection(&self.socket, ip_addr);

        self.request_nodes();


        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        self.handle_message(msg);

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

    fn handle_message(&mut self, msg: Message) {
        match msg {
            Message::Text(s) => {
                println!("{:?}", s);
                let parsed: LC::Message = serde_json::from_str(&s).unwrap();

                match parsed.typ {
                    MessageType::Request => {
                        match parsed.action {
                            Some(action) => {
                                if(action == "get_nodes") {
                                    let connections = self.node.get_connections();
                                    let resp_data: String = connections.into_iter().collect();

                                    let response = json!(LC::Message {
                                        typ: LC::MessageType::Response,
                                        action: None,
                                        data: Some(resp_data)
                                    });

                                    self.socket.send(response.to_string());
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}

fn run_server(node: Node) {

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

fn run_client(node: Node) {
    let GOD: String = "ws://10.8.57.232:9001".to_string();

    spawn(move || {
        connect(GOD, |socket| {
            Server {
                socket: socket,
                node: node.clone()
            }
        }).unwrap()
    });
}

fn main() {
    let node: Node = Default::default();

    run_server(node.clone());

    run_client(node.clone());

    loop {
    }
}
