use crate::blockchain::balance_manager::BalanceManager;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::BlockChain;
use crate::blockchain::blockmap::BlockMap;
use crate::transaction::SignedTransaction;
use crate::LC;
use serde_json::json;
use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use ws::Sender;

type Connections = Arc<Mutex<HashMap<String, Sender>>>;

pub struct Node {
    pub connections: Connections,
    pub chain: BlockChain,
    pub blocks: BlockMap,
    pub balance_manager: BalanceManager,
    pub worker_tx: mpsc::Sender<(SignedTransaction, String)>,
    pub threshold: [u8; 32],
}

impl Clone for Node {
    fn clone(&self) -> Self {
        Node {
            connections: Arc::clone(&self.connections),
            chain: self.chain.clone(),
            blocks: self.blocks.clone(),
            balance_manager: self.balance_manager.clone(),
            worker_tx: self.worker_tx.clone(),
            threshold: self.threshold.clone(),
        }
    }
}

impl Node {
    pub fn create(tx: mpsc::Sender<(SignedTransaction, String)>, threshold: [u8; 32]) -> Self {
        Node {
            connections: Arc::new(Mutex::new(HashMap::new())),
            chain: Default::default(),
            blocks: Default::default(),
            balance_manager: Default::default(),
            worker_tx: tx,
            threshold,
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

    pub fn add_block(&mut self, block: Block) {
        let block_hash = block.hash();
        println!("Adding block: {}", block_hash);
        let result = self.balance_manager.process_transaction(&block.tx.data);

        if let Err(a) = result {
            println!("{}", a.to_string());
            return;
        } else {
            self.blocks.insert(block_hash.clone(), block);
            self.chain.push_back(block_hash);
        }
    }

    pub fn add_and_broadcast_block(&mut self, block: Block) {
        self.add_block(block.clone());

        let json_block = json!(block).to_string();
        let block_msg = LC::Message {
            typ: LC::MessageType::CreatedBlock,
            action: None,
            data: Some(json_block),
        };

        let json_msg = json!(block_msg).to_string();

        self.broadcast(json_msg);
    }
}
