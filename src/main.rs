use std::net::TcpListener;
use std::thread::spawn;
use url::Url;
use tungstenite::{accept, connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::collections::HashMap;
use serde_json::Value;
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

/// A WebSocket echo server
fn main() {
    //let mut connections = HashMap::new();

    let ERIC_ADDR: String = String::from("10.8.57.232:9001");
    let localhost = "0.0.0.0:9001";
    println!("wh");
    let server = TcpListener::bind(localhost).unwrap();
    println!("wh");
// spawn(|| {
// });

    println!("Run server!");
    spawn(move || {
        for stream in server.incoming() {
            spawn(move || {
                let mut websocket = accept(stream.unwrap()).unwrap();
                println!("{:?}", websocket);
// websocket.write_message(Message::text("cool cool"));

                loop {
                    let msg = websocket.read_message().unwrap();

// We do not want to send back ping/pong messages.
                    if msg.is_binary() || msg.is_text() {
                        websocket.write_message(Message::text("that's dumb")).unwrap();
                    }
                }
            });
        }
    });

    let ERIC = "ws://10.8.57.232:9001";
    let (mut socket, response) = connect(Url::parse("ws://10.8.57.232:9001").unwrap()).unwrap();
    connections[ERIC] = socket;

    println!("{:?}", response);

    let mut i = 0;
    // loop {
    i += 1;
    socket.write_message(Message::text(i.to_string())).unwrap();

    let msg = socket.read_message().unwrap();

    if msg.is_text() {
        let text = msg.into_text().unwrap();
        let v: LC::Message = serde_json::from_str(&text).unwrap();

        println!("{:?}", v);

    }

// We do not want to send back ping/pong messages.
//     if msg.is_binary() || msg.is_text() {
//         println!("{}", &msg.into_text().unwrap());
//         socket.write_message(Message::text((i.to_string()))).unwrap();
//     }
    // }
}

fn get_nodes() {}
