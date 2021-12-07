use crate::ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};
use std::thread::spawn;
use crate::node as node;
use crate::server as server;

pub fn run_server(node: node::Node) {
    let localhost = "0.0.0.0:9001";

    spawn(move || listen(localhost,
                         |socket| {
                             server::Server {
                                 socket: socket,
                                 node: node.clone(),
                             }
                         }).unwrap()
    );
    println!("Running server!");
}

pub fn connect_to_ip(ip: String, node: node::Node) {
    spawn(move || {
        connect(ip, |socket| {
            server::Server {
                socket: socket,
                node: node.clone(),
            }
        }).unwrap()
    });
}


impl Handler for server::Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        let ip_addr = shake.remote_addr()?.unwrap();
        self.node.add_new_connection(&self.socket, ip_addr);

        self.request_nodes();


        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> Result<()> {
        self.handle_data_received(msg);

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("Client connection closing: {}", reason)
    }
}
