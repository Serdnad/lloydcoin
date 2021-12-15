use super::block::Block;
use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use std::fmt;

type Hash = String;

pub struct BlockMap {
    blockmap_mutex: Arc<Mutex<HashMap<Hash, Block>>>,
}

impl BlockMap {
    pub fn insert(&mut self, key: String, value: Block) {
        let mut blockmap = self.blockmap_mutex.lock().unwrap();
        blockmap.insert(key, value);
    }

    pub fn get(&self, key: &String) -> std::option::Option<Block> {
        let blockmap = self.blockmap_mutex.lock().unwrap();
        let value = blockmap.get(key);

        if let Some(a) = value {
            Some(Block {
                prev_hash: a.prev_hash.clone(),
                tx: a.tx.clone(),
                nonce: a.nonce.clone(),
            })
        } else {
            None
        }
    }
}

impl Default for BlockMap {
    fn default() -> Self {
        BlockMap {
            blockmap_mutex: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Clone for BlockMap {
    fn clone(&self) -> Self {
        BlockMap {
            blockmap_mutex: Arc::clone(&self.blockmap_mutex),
        }
    }
}

impl fmt::Debug for BlockMap {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let blockmap = self.blockmap_mutex.lock().unwrap();

        fmt.debug_map()
            .entries(blockmap.iter().map(|(ref k, ref v)| (k.clone(), v.clone())))
            .finish()
    }
}
