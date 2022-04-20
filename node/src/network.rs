use crate::node;
use crate::server;
use crate::ws::{connect, listen, CloseCode, Handler, Handshake, Result};
use std::thread::spawn;

/// Creates a thread that listens for incoming connections.
pub fn run_server(node: node::Node,) {
    let localhost = "0.0.0.0:9001";

    spawn(move || {
        listen(localhost, |socket| server::Server {
            socket: socket,
            node: node.clone(),
        })
        .unwrap()
    });
    println!("Running server!");
}

/// Creates a thread that connects to the given ip.
pub fn connect_to_ip(ip: String, node: node::Node) {
    spawn(move || {
        connect(ip, |socket| server::Server {
            socket: socket,
            node: node.clone(),
        })
        .unwrap()
    });
}

impl Handler for server::Server {
    /// Callback for a new connection.
    ///
    /// Adds the new connection to the list of connections in the Node.
    /// Initiates synchronizing list of connections and blockchain with the new Node.
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let ip_addr = shake.remote_addr()?.unwrap();
        self.node.add_new_connection(&self.socket, ip_addr);

        self.request_nodes();

        self.request_new_blocks();

        Ok(())
    }

    /// Callback for any data sent from the connection.
    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        self.handle_data_received(msg);

        Ok(())
    }

    /// Callback for the connection closing.
    fn on_close(&mut self, code: CloseCode, reason: &str) {
        // TODO: Remove the connection from the list of connections in the Node
        println!("Client connection closing: {}", reason)
    }
}
