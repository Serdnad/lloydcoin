use std::sync::{Mutex, Arc};
use ws::{connect, listen, CloseCode, Handler, Message, Sender, Handshake};
use std::collections::LinkedList;
use std::collections::HashMap;
use crate::blockchain::{BlockMap};
use crate::blockchain::balance_manager::BalanceManager;
use crate::blockchain::blockchain::{BlockChain};
use crate::transaction::TransactionData;

type Connections = Arc<Mutex<HashMap<String, Sender>>>;

pub struct Node {
    pub connections: Connections,
    pub chain: BlockChain,
    pub blocks: BlockMap,
    pub balance_manager: BalanceManager
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            connections: Arc::clone(&self.connections),
            chain: self.chain.clone(),
            blocks: Default::default(),
            balance_manager: self.balance_manager.clone()
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            connections: Arc::new(Mutex::new(HashMap::new())),
            chain: Default::default(),
            blocks: Default::default(),
            balance_manager: Default::default()
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
