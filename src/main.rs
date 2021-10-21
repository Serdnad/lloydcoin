mod node;

#[macro_use]
extern crate lazy_static;

use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use url::Url;
use tungstenite::{accept, connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::collections::HashMap;
use serde_json::{Value, json};
use serde::{Deserialize, Serialize};
use crate::LC::MessageType;

use std::sync::{Mutex, Arc};


lazy_static! {
    static ref SOCKETS: Mutex<HashMap<String, WebSocket<TcpStream>>> = {
        Mutex::new(HashMap::new())
    };
}


mod LC {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum MessageType {
        Request,
        Response,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Message<'a> {
        pub typ: MessageType,
        pub action: Option<&'a str>,
        pub data: Option<&'a str>,
    }
}


fn run_server() {
    let mut sockets = HashMap::new();

    let localhost = "0.0.0.0:9001";
    let server = TcpListener::bind(localhost).unwrap();

    println!("Running server!");

    spawn(move || {
        for stream in server.incoming() {
            spawn(move || {
                let websocket = accept(stream.unwrap()).unwrap();
                println!("Something connected to server");
                let a = websocket.get_ref().peer_addr().unwrap().to_string();
                sockets.insert(String::from(a), Arc::new(websocket));
                // SOCKETS.lock().unwrap().insert(a, websocket);

                loop {
                    let msg = sockets.get(&a).unwrap().read_message().unwrap().into_text().unwrap(); // .read_message().unwrap().into_text().unwrap();


                    let s: LC::Message = serde_json::from_str(&msg).unwrap();

                    match s.typ {
                        MessageType::Request => {
                            match s.action {
                                Some("get_nodes") => {
                                    println!("YO!");


                                    // let a: Vec<&String> = sockets.get_ .keys().into_iter().collect();
                                    println!("{:?}", a);


                                    let resp = json!(LC::Message {
                                    typ: LC::MessageType::Response,
                                    action: None,
                                    data: Some(json!(a).to_string().as_str())
                                    });
                                    sockets.get(&a).unwrap().write_message(Message::text(resp.to_string()));
                                }
                                _ => {}
                            }
                        }
                        MessageType::Response => {}
                    }
                }

                //println!("{:?}", websocket);

                let return_msg: Value = json!(LC::Message {
                    typ: LC::MessageType::Request,
                    action: Some("get_nodes"),
                    data: None,
                });
                websocket.write_message(Message::text(return_msg.to_string()));
            });
        }
    });
}

fn run_client() {
    let GOD: String = "ws://10.8.4.155:9001".to_string();
    let (mut socket, response) = connect(Url::parse(&GOD).unwrap()).unwrap();

    // spawn(|| {
    //     let msg = socket.read_message().unwrap();
    //     println!("{:?}", msg);
    // });

    let request = json!(LC::Message {
                    typ: LC::MessageType::Request,
                    action: Some("get_nodes"),
                    data: None,
                });
    socket.write_message(Message::text(request.to_string()));

    // has to be a FAST reader
    let msg = socket.read_message().unwrap();
    println!("{:?}", msg);


    // loop {
    //     let response = socket.read_message();
    //
    //     if response.is_ok() {
    //         let msg = response.unwrap();
    //         if msg.is_text() {
    //             let text = msg.into_text().unwrap();
    //             let v: LC::Message = serde_json::from_str(&text).unwrap();
    //
    //
    //             println!("{:?}", v);
    //         }
    //     }
    // }
}

/// A WebSocket echo server
fn main() {
    run_server();

    run_client();
}