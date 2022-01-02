use crate::blockchain::block::Block;
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

        // If the most_recent hash is never found then it will give the whole
        // blockchain so GENESIS will be the last element.
        // This means that the other node is more up-to-date, so send nothing.
        if let Some(last) = block_hashes.last() {
            if last == "GENESIS" {
                return None;
            }
        }

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

    pub fn handle_get_blocks_response(
        &mut self,
        blocks: Vec<Block>,
    ) -> Result<Option<String>, String> {
        self.validate_blocks(&blocks)?;
        self.add_blocks(blocks);

        Ok(None)
    }
}
