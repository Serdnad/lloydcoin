use crate::blockchain::balance_manager::BalanceManager;
use crate::blockchain::block::Block;
use crate::blockchain::blockchain::BlockChain;
use crate::blockchain::blockmap::BlockMap;
use crate::server::handlers;
use crate::server::Server;
use crate::LC;
use serde_json::json;

impl Server {
    pub fn request_new_blocks(&self) {
        let most_recent = self.node.chain.back();
        println!("Sending request with most recent {:?}", most_recent);

        let request = json!(LC::Message {
            typ: LC::MessageType::Request,
            action: Some("get_blocks".to_string()),
            data: most_recent
        });

        self.socket.send(request.to_string());
    }

    pub fn handle_get_blocks_request(&mut self, most_recent: String) -> Option<String> {
        println!(
            "Received get blocks request with most recent {:?}",
            most_recent
        );
        let get_block_hashes =
            |iter: std::collections::linked_list::Iter<'_, String>| -> Vec<String> {
                iter.rev()
                    .cloned()
                    .take_while(|hash| hash.to_string() != most_recent)
                    .collect::<Vec<String>>()
            };

        let block_hashes = self.node.chain.iter(get_block_hashes);

        let mut blocks = Vec::new();
        for hash in block_hashes {
            blocks.push(self.node.blocks.get(&hash));
        }

        println!("Found {:?} blocks since then", blocks.len());

        let response = json!(LC::Message {
            typ: LC::MessageType::Response,
            action: Some("get_blocks".to_string()),
            data: Some(json!(blocks).to_string())
        });

        Some(response.to_string())
    }

    fn validate_blocks(&self, blocks: &Vec<Block>) -> Result<(), String> {
        let mut prev_hash = self.node.chain.back().unwrap();
        let threshold = self.node.threshold;
        let mut balance_manager = self.node.balance_manager.clone();

        for block in blocks.iter().rev() {
            handlers::validate_block(prev_hash, threshold, &balance_manager, &block)?;
            balance_manager.process_transaction(&block.tx.data)?;
            prev_hash = block.hash();
        }

        Ok(())
    }

    fn add_blocks(&mut self, blocks: Vec<Block>) {
        for block in blocks.into_iter().rev() {
            self.node.add_block(block);
        }
    }

    fn is_subchain(&mut self, blocks: &Vec<Block>) -> bool {
        let most_recent_received = blocks.last().unwrap().prev_hash.clone();

        let check_for_most_recent =
            |iter: std::collections::linked_list::Iter<'_, String>| -> bool {
                for hash in iter.rev() {
                    if hash == &most_recent_received {
                        return true;
                    }
                }
                false
            };

        self.node.chain.iter(check_for_most_recent)
    }

    fn validate_entire_chain(&mut self, blocks: &Vec<Block>) -> Result<BalanceManager, String> {
        let mut prev_hash = "GENESIS".to_string();
        let threshold = self.node.threshold;
        let mut balance_manager = BalanceManager::default();

        for block in blocks.iter().rev() {
            handlers::validate_block(prev_hash, threshold, &balance_manager, &block)?;
            balance_manager.process_transaction(&block.tx.data)?;
            prev_hash = block.hash();
        }

        Ok(balance_manager)
    }

    fn replace_chain(&mut self, blocks: Vec<Block>, balance_manager: BalanceManager) {
        self.node.balance_manager = balance_manager;

        let mut blockchain = BlockChain::default();
        let mut block_map = BlockMap::default();
        for block in blocks.into_iter().rev() {
            blockchain.push_back(block.hash());
            block_map.insert(block.hash(), block);
        }

        self.node.blocks = block_map;
        self.node.chain = blockchain;
    }

    pub fn handle_get_blocks_response(
        &mut self,
        blocks: Vec<Block>,
    ) -> Result<Option<String>, String> {
        let last = blocks.last();

        // If the last block (called first_block) is "GENSIS"
        // then the whole chain was transferred over.
        // There are two cases: either this transferred chain is a subchain or it is a different
        // chain
        // If it's a subchain, who cares.
        // If it's longer, then replace the current chain
        if let Some(first_block) = last {
            if first_block.prev_hash == "GENESIS" {
                if self.is_subchain(&blocks) {
                    return Ok(None);
                }

                let balance_manager = self.validate_entire_chain(&blocks)?;
                if blocks.len() > self.node.chain.len() {
                    self.replace_chain(blocks, balance_manager);
                } else {
                    return Ok(None);
                }
            } else {
                self.validate_blocks(&blocks)?;
                self.add_blocks(blocks);
            }
        }

        Ok(None)
    }
}
