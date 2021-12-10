use std::collections::HashMap;
use crate::transaction::TransactionData;

type Balance = u64;

pub struct BalanceManager {
    pub accounts: HashMap<String, Balance>,
}

impl BalanceManager {
    /// Get an account's balance. Defaults to 0 for accounts with no transactions.
    pub fn get_balance(&self, account: &str) -> Balance {
        *self.accounts.get(account).unwrap_or(&0)
    }

    /// Verify that all current balances allow the given transaction to be executed, and update
    /// balances accordingly if so.
    pub fn process_transaction(&mut self, tx: TransactionData) -> Result<(), &str> {
        let sender_balance = self.get_balance(&tx.sender_key);
        let recipient_balance = self.get_balance(&tx.receiver_key);

        if tx.amount > sender_balance {
            Err("insufficient funds")
        } else {
            self.accounts.insert(tx.sender_key, sender_balance - tx.amount);
            self.accounts.insert(tx.receiver_key, recipient_balance + tx.amount);

            Ok(())
        }
    }
}

impl Default for BalanceManager {
    fn default() -> Self {
        BalanceManager {
            accounts: HashMap::new()
        }
    }
}
