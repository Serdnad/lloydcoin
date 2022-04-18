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
    // The hash of a block must be below this threshold
    // More 0x00 means harder
    let threshold: [u8; 32] = [
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];

    // The worker is in a separate thread and uses tx to communicate here
    let (tx, rx) = mpsc::channel();

    // The worker returns a tx that is used by node to send work to it
    let worker_tx = worker::create_worker(tx, threshold);

    let mut node: node::Node = Node::create(worker_tx, threshold);

    network::run_server(node.clone());

    let GOD: String = "ws://10.8.57.232:9001".to_string();
    network::connect_to_ip(GOD, node.clone());

    // Blocks that the worker completes are sent here and then sent to a node
    loop {
        let new_block = rx.recv().unwrap();

        node.add_and_broadcast_block(new_block);
    }
}
