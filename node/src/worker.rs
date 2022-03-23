use crate::blockchain::block::Block;
use crate::transaction::SignedTransaction;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Tx used by Node to send blocks that need to be mined.
type InfoTransmitter = mpsc::Sender<(SignedTransaction, String)>;

/// Tx used by Worker to send mined blocks to main.rs.
type BlockTransmitter = mpsc::Sender<Block>;

/// Rx used by Worker to received blocks that need to be mined.
type Receiver = mpsc::Receiver<(SignedTransaction, String)>;

/// Checks if the hash of the block is lower than the threshold.
pub fn does_nonce_work(block: &Block, threshold: [u8; 32]) -> bool {
    let hash = hex::decode(block.hash()).unwrap();

    let mut less_than = true;
    for (i, elem) in hash.iter().enumerate() {
        if elem > &threshold[i] {
            less_than = false;
            break;
        }
    }
    if less_than {
        return true;
    } else {
        return false;
    }
}

/// Receives blocks from a Node and then mines them.
fn do_work(node_tx: BlockTransmitter, rx: Receiver, threshold: [u8; 32]) {
    let mut rng = rand::thread_rng();

    loop {
        let (transaction, prev_hash) = rx.recv().unwrap();

        loop {
            // TODO: Random is bad. Do it sequentially.
            // Maybe have multiple threads start a different points.
            let nonce: u64 = rng.gen();

            let block = Block {
                tx: transaction.clone(),
                prev_hash: prev_hash.clone(),
                nonce,
            };

            if does_nonce_work(&block, threshold) {
                node_tx.send(block);

                println!("Nonce found");

                break;
            }

            thread::sleep(Duration::from_millis(5));
        }
    }
}

/// Creates thread for the worker to receives and mine blocks.
///
/// Also returns tx for Node to use to send blocks.
pub fn create_worker(node_tx: BlockTransmitter, threshold: [u8; 32]) -> InfoTransmitter {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        do_work(node_tx, rx, threshold);
    });

    tx
}
