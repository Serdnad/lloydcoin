use crate::blockchain::block::Block;
use crate::transaction::SignedTransaction;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

type InfoTransmitter = mpsc::Sender<(SignedTransaction, String)>;
type BlockTransmitter = mpsc::Sender<Block>;
type Receiver = mpsc::Receiver<(SignedTransaction, String)>;

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

fn do_work(node_tx: BlockTransmitter, rx: Receiver, threshold: [u8; 32]) {
    let mut rng = rand::thread_rng();

    loop {
        let (transaction, prev_hash) = rx.recv().unwrap();

        loop {
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

pub fn create_worker(node_tx: BlockTransmitter, threshold: [u8; 32]) -> InfoTransmitter {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        do_work(node_tx, rx, threshold);
    });

    tx
}
