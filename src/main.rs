use std::net::TcpListener;
use std::thread::spawn;
use url::Url;
use tungstenite::{accept, connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::collections::HashMap;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};

mod LC {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub enum MessageType {
        Request,
        Response,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message {
        pub typ: MessageType,
        pub action: String,
    }
}

fn run_server() {
    let localhost = "0.0.0.0:9001";

    let server = TcpListener::bind(localhost).unwrap();

    println!("Running server!");

    spawn(move || {
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                println!("Something connected to server");
                //println!("{:?}", websocket);

                let return_msg: Value = json!(LC::Message {
                    typ: LC::MessageType::Request,
                    action: "get nodes".to_string()
                });
                websocket.write_message(Message::text(return_msg.to_string()));
            });
        }
    });
}

fn run_client() {
    let ERIC: String = "ws://10.8.57.232:9001".to_string();
    let (mut socket, response) = connect(Url::parse(&ERIC).unwrap()).unwrap();

    println!("{:?}", response);

    loop {
        let response = socket.read_message();

        if(response.is_ok()) {
            let msg = response.unwrap();
            if msg.is_text() {
                let text = msg.into_text().unwrap();
                let v: LC::Message = serde_json::from_str(&text).unwrap();


                println!("{:?}", v);
            }
        }
    }
}

/// A WebSocket echo server
fn main() {
    run_server();

    run_client();
}

fn get_nodes() {}
