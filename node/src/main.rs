use std::thread::spawn;

extern crate ws;
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};

mod LC;
mod node;
mod server;

fn run_server(node: node::Node) {

    let localhost = "0.0.0.0:9001";

    spawn(move || listen(localhost, 
        |socket| {
            server::Server {
                socket: socket,
                node: node.clone()
            }
        }).unwrap()
    );
    println!("Running server!");
}

fn main() {
    let node: node::Node = Default::default();

    run_server(node.clone());

    let GOD: String = "ws://10.8.57.232:9001".to_string();
    server::connect_to_ip(GOD, node.clone());

    loop {
    }
}
