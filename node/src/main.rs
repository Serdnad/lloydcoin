extern crate ws;

use std::thread::spawn;

use ws::{CloseCode, connect, Handler, Handshake, listen, Message, Result, Sender};

use crate::node::Node;


mod LC;
mod node;
mod server;
mod network;
mod transaction;
mod node_sharing;
mod blockchain;

fn main() {
    let mut node: node::Node = Default::default();
    node.chain.push_back(String::from("GENESIS"));

    network::run_server(node.clone());

    let GOD: String = "ws://10.8.4.155:9001".to_string();
    network::connect_to_ip(GOD, node.clone());

    loop {}
}
