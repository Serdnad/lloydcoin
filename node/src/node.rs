use std::sync::{Mutex, Arc};
use ws::{connect, listen, CloseCode, Handler, Message, Result, Sender, Handshake};
use std::collections::LinkedList;
use std::collections::HashMap;
use crate::blockchain::{BlockChain, BlockMap};

type PublicKeyNum = u32;

pub struct Vertebra {
    balance: (PublicKeyNum, u32),
}

type Connections = Arc<Mutex<HashMap<String, Sender>>>;
type Snake = Arc<Mutex<LinkedList<Vertebra>>>;

pub struct Node {
    pub connections: Connections,
    pub snake: Snake,
    pub chain: BlockChain,
    pub blocks: BlockMap,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            connections: Arc::clone(&self.connections),
            snake: Arc::new(Mutex::new(Default::default())),
            chain: self.chain.clone(),
            blocks: Default::default(),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            connections: Arc::new(Mutex::new(HashMap::new())),
            snake: Arc::new(Mutex::new(Default::default())),
            chain: Default::default(),
            blocks: Default::default(),
        }
    }
}

impl Node {
    pub fn add_new_connection(&mut self, socket: &Sender, ip_addr: String) {
        let mut map = self.connections.lock().unwrap();
        map.insert(ip_addr, socket.clone());
    }

    pub fn get_connections(&mut self) -> Vec<String> {
        let mut map = self.connections.lock().unwrap();
        map.keys().cloned().collect()
    }

    pub fn contains_ip(&mut self, ip: &str) -> bool {
        let mut map = self.connections.lock().unwrap();
        map.contains_key(ip)
    }
}
