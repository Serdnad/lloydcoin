use std::collections::HashMap;
use std::collections::LinkedList;

use std::sync::{Arc, Mutex};

use std::fmt;

type Hash = String;

pub struct BlockChain {
    chain_mutex: Arc<Mutex<LinkedList<Hash>>>,
}

impl BlockChain {
    pub fn back(&self) -> Option<Hash> {
        let chain = self.chain_mutex.lock().unwrap();
        let back = chain.back();

        if let Some(a) = back {
            Some(a.clone())
        } else {
            None
        }
    }

    pub fn push_back(&mut self, element: Hash) {
        let mut chain = self.chain_mutex.lock().unwrap();
        chain.push_back(element);
    }

    pub fn iter<R, T: Fn(std::collections::linked_list::Iter<'_, Hash>) -> R>(&self, func: T) -> R {
        let chain = self.chain_mutex.lock().unwrap();
        func(chain.iter())
    }
}

impl Default for BlockChain {
    fn default() -> Self {
        BlockChain {
            chain_mutex: Arc::new(Mutex::new(LinkedList::new())),
        }
    }
}

impl Clone for BlockChain {
    fn clone(&self) -> Self {
        BlockChain {
            chain_mutex: Arc::clone(&self.chain_mutex),
        }
    }
}

impl fmt::Debug for BlockChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let chain = self.chain_mutex.lock().unwrap();
        f.debug_list().entries(chain.iter()).finish()
    }
}
