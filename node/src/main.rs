extern crate ws;

use std::thread::spawn;

use ws::{connect, listen, CloseCode, Handler, Handshake, Message, Result, Sender};

use crate::node::Node;
use std::sync::mpsc;

mod LC;
mod blockchain;
mod network;
mod node;
mod node_sharing;
mod server;
mod transaction;
mod update_blockchain;
mod worker;

fn main() {
    let threshold: [u8; 32] = [
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];

    let (tx, rx) = mpsc::channel();

    let worker_tx = worker::create_worker(tx, threshold);

    let mut node: node::Node = Node::create(worker_tx, threshold);
    node.chain.push_back(String::from("GENESIS"));

    network::run_server(node.clone());

    let GOD: String = "ws://192.168.4.26:9001".to_string();
    network::connect_to_ip(GOD, node.clone());

    loop {
        let new_block = rx.recv().unwrap();

        node.add_and_broadcast_block(new_block);
    }
}
