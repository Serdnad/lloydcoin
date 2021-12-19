use crate::blockchain::balance_manager::BalanceManager;
use crate::blockchain::blockchain::BlockChain;
use crate::blockchain::blockmap::BlockMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::Sender;

type Connections = Arc<Mutex<HashMap<String, Sender>>>;

pub struct Node {
    pub connections: Connections,
    pub chain: BlockChain,
    pub blocks: BlockMap,
    pub balance_manager: BalanceManager,
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            connections: Arc::clone(&self.connections),
            chain: self.chain.clone(),
            blocks: self.blocks.clone(),
            balance_manager: self.balance_manager.clone(),
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Node {
            connections: Arc::new(Mutex::new(HashMap::new())),
            chain: Default::default(),
            blocks: Default::default(),
            balance_manager: Default::default(),
        }
    }
}

impl Node {
    pub fn add_new_connection(&mut self, socket: &Sender, ip_addr: String) {
        let mut map = self.connections.lock().unwrap();
        map.insert(ip_addr, socket.clone());
    }

    pub fn get_connections(&mut self) -> Vec<String> {
        let map = self.connections.lock().unwrap();
        map.keys().cloned().collect()
    }

    pub fn broadcast(&mut self, data: String) {
        let map = self.connections.lock().unwrap();
        for socket in map.values() {
            socket.send(data.clone());
        }
    }

    pub fn contains_ip(&mut self, ip: &str) -> bool {
        let map = self.connections.lock().unwrap();
        map.contains_key(ip)
    }
}
