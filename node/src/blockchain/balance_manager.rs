use std::collections::HashMap;
use crate::transaction::TransactionData;
use std::sync::{Mutex, Arc};

type Balance = u64;

pub struct BalanceManager {
    accounts_mutex: Arc<Mutex<HashMap<String, Balance>>>
}

impl BalanceManager {
    /// Get an account's balance. Defaults to 0 for accounts with no transactions.
    pub fn get_balance(&mut self, account: &str) -> Balance {
        let mut accounts = self.accounts_mutex.lock().unwrap();
        *accounts.get(account).unwrap_or(&0)
    }

    /// Verify that all current balances allow the given transaction to be executed, and update
    /// balances accordingly if so.
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
            accounts_mutex: Arc::new(Mutex::new(HashMap::new()))
        };
        new.accounts_mutex.lock().unwrap().insert(
            "02eed7e3ce21528429310300046cd3d41434bcaac7c78bb930735c7913b52eb79d".to_string(), 
            500);
        new
    }
}

impl Clone for BalanceManager {
    fn clone(&self) -> Self {
        BalanceManager {
            accounts_mutex: Arc::clone(&self.accounts_mutex)
        }
    }
}
