use std::thread::spawn;

extern crate ws;
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};

mod LC;
mod node;
mod server;
mod network;
mod transaction;
mod node_sharing;

fn main() {
    let node: node::Node = Default::default();

    network::run_server(node.clone());

    let GOD: String = "ws://10.8.57.232:9001".to_string();
    network::connect_to_ip(GOD, node.clone());

    loop {
    }
}
