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

fn try_nonce(
    tx: SignedTransaction,
    prev_hash: String,
    nonce: u64,
    threshold: [u8; 32],
) -> Option<Block> {
    let block = Block {
        tx: tx.clone(),
        prev_hash: prev_hash.clone(),
        nonce,
    };

    let hash = hex::decode(block.hash()).unwrap();

    let mut less_than = true;
    for (i, elem) in hash.iter().enumerate() {
        if elem > &threshold[i] {
            less_than = false;
            break;
        }
    }
    if less_than {
        return Some(block);
    } else {
        return None;
    }
}

fn do_work(node_tx: BlockTransmitter, rx: Receiver) {
    let mut rng = rand::thread_rng();

    let threshold: [u8; 32] = [
        0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        0xff, 0xff,
    ];

    loop {
        let (transaction, prev_hash) = rx.recv().unwrap();

        loop {
            let nonce: u64 = rng.gen();

            let maybe_block = try_nonce(transaction.clone(), prev_hash.clone(), nonce, threshold);

            if let Some(block) = maybe_block {
                node_tx.send(block);

                println!("Nonce found");

                break;
            }

            thread::sleep(Duration::from_millis(5));
        }
    }
}

pub fn create_worker(node_tx: BlockTransmitter) -> InfoTransmitter {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        do_work(node_tx, rx);
    });

    tx
}
