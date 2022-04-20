use crate::transaction::TransactionData;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const GENESIS_WALLET: &str = "02eed7e3ce21528429310300046cd3d41434bcaac7c78bb930735c7913b52eb79d";
const GENESIS_AMOUNT: u64 = 1.960e9 as u64; // Lloyd was built in 1960
// TODO: or should we make, like, 69 million ?

type Balance = u64;

pub struct BalanceManager {
    accounts_mutex: Arc<Mutex<HashMap<String, Balance>>>,
}

impl BalanceManager {
    /// Get an account's balance. Defaults to 0 for accounts with no transactions.
    pub fn get_balance(&self, account: &str) -> Balance {
        let accounts = self.accounts_mutex.lock().unwrap();
        *accounts.get(account).unwrap_or(&0)
    }

    /// Checks if the sender for the transaction has sufficient funds.
    ///
    /// # Errors
    /// The sender does not have sufficient funds.
    pub fn sufficient_funds(&self, tx: &TransactionData) -> Result<(), &str> {
        let sender_balance = self.get_balance(&tx.sender_key);

        if tx.amount > sender_balance {
            return Err("insufficient funds");
        }

        Ok(())
    }

    /// Update the receiver and sender funds based on the transaction
    ///
    /// # Errors
    /// The sender has insufficient funds.
    pub fn process_transaction(&mut self, tx: &TransactionData) -> Result<(), &str> {
        let sender_balance = self.get_balance(&tx.sender_key);
        let recipient_balance = self.get_balance(&tx.receiver_key);

        if tx.amount > sender_balance {
            Err("insufficient funds")
        } else {
            let mut accounts = self.accounts_mutex.lock().unwrap();

            accounts.insert(tx.sender_key.clone(), sender_balance - tx.amount);
            accounts.insert(tx.receiver_key.clone(), recipient_balance + tx.amount);

            Ok(())
        }
    }
}

impl Default for BalanceManager {
    fn default() -> Self {
        let new = BalanceManager {
            accounts_mutex: Arc::new(Mutex::new(HashMap::new())),
        };

        // the public key of the initial account with all coins
        new.accounts_mutex.lock().unwrap().insert(
            GENESIS_WALLET.to_string(),
            GENESIS_AMOUNT,
        );
        new
    }
}

impl Clone for BalanceManager {
    fn clone(&self) -> Self {
        BalanceManager {
            accounts_mutex: Arc::clone(&self.accounts_mutex),
        }
    }
}
