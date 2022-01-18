use serde::{Serialize, Serializer};
use std::collections::HashMap;
use std::vec::Vec;

use crate::transaction::*;
use crate::transaction_error::*;

/// Represents a client account with id, amounts, status, and previous transactions.
#[derive(Clone, Debug, Serialize)]
pub struct Client {
    /// Unique client ID
    #[serde(rename = "client")]
    pub id: u16,

    /// Funds available for withdrawal.
    #[serde(serialize_with = "four_decimal_serializer")]
    available: f64,

    /// Funds held in dispute.
    #[serde(serialize_with = "four_decimal_serializer")]
    held: f64,

    /// Total funds in account.
    #[serde(serialize_with = "four_decimal_serializer")]
    total: f64,

    /// Locked is true if a chargeback has been issued.
    locked: bool,

    /// A log of prevous transactions, grouped by transaction ID.
    #[serde(skip_serializing)]
    transactions: HashMap<u32, Vec<Transaction>>,
}

/// Create an new client with default settings, then apply their first transation.
pub fn initialize_client(transaction: Transaction) -> Result<Client, TransactionError> {
    let client = Client {
        id: transaction.client_id,
        available: 0.0,
        held: 0.0,
        total: 0.0,
        locked: false,
        transactions: HashMap::new(),
    };
    Ok(client.apply_transaction(transaction)?)
}

impl Client {
    /// Try to apply the given tranasaction to the client, and if successful return the updated client.
    /// May produce a TransactionError if the transaction breaks any rules.
    pub fn apply_transaction(
        mut self,
        transaction: Transaction,
    ) -> Result<Client, TransactionError> {
        if self.locked {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::AccountLocked,
                transaction: transaction,
                client: self,
            });
        }
        self = match transaction.transaction_type {
            TransactionType::Deposit => self.apply_deposit(transaction)?,
            TransactionType::Withdrawal => self.apply_withdrawal(transaction)?,
            TransactionType::Dispute => self.apply_dispute(transaction)?,
            TransactionType::Resolve => self.apply_resolve(transaction)?,
            TransactionType::Chargeback => self.apply_chargeback(transaction)?,
        };
        Ok(self)
    }

    /// If the given amount is Some(positive number), add it to available and total funds.
    fn apply_deposit(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(amount) = transaction.amount {
            if amount <= 0.0 {
                return Err(TransactionError {
                    error_type: TransactionErrorTypes::NonPositiveAmount,
                    transaction: transaction,
                    client: self,
                });
            }
            self.available += amount;
            self.total += amount;
        } else {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::MissingRequiredAmount,
                transaction: transaction,
                client: self,
            });
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given amount is Some(positive number) and there are enough available funds, subtract it from available and total funds.
    fn apply_withdrawal(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(amount) = transaction.amount {
            if amount <= 0.0 {
                return Err(TransactionError {
                    error_type: TransactionErrorTypes::NonPositiveAmount,
                    transaction: transaction,
                    client: self,
                });
            }
            if amount > self.available {
                return Err(TransactionError {
                    error_type: TransactionErrorTypes::InsufficientFunds,
                    transaction: transaction,
                    client: self,
                });
            }
            self.available -= amount;
            self.total -= amount;
        } else {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::MissingRequiredAmount,
                transaction: transaction,
                client: self,
            });
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given transaction ID exists in the log, move the amount specified in that deposit from available to held.
    fn apply_dispute(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(_) = transaction.amount {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::HasMeaninglessAmount,
                transaction: transaction,
                client: self,
            });
        }
        if let Some(related_transactions) = self.transactions.get(&transaction.id) {
            let amount = related_transactions[0].amount.unwrap();
            self.available -= amount;
            self.held += amount;
        } else {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::InvalidIdReferenced,
                transaction: transaction,
                client: self,
            });
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given transaction ID exists in the log, move the amount specified in that deposit from held to available.
    fn apply_resolve(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(_) = transaction.amount {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::HasMeaninglessAmount,
                transaction: transaction,
                client: self,
            });
        }
        if let Some(related_transactions) = self.transactions.get(&transaction.id) {
            let amount = related_transactions[0].amount.unwrap();
            self.held -= amount;
            self.available += amount;
        } else {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::InvalidIdReferenced,
                transaction: transaction,
                client: self,
            });
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given transaction ID exists in the log, subrtract the amount specified in that deposit from held and total, then lock the account.
    fn apply_chargeback(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(_) = transaction.amount {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::HasMeaninglessAmount,
                transaction: transaction,
                client: self,
            });
        }
        if let Some(related_transactions) = self.transactions.get(&transaction.id) {
            let amount = related_transactions[0].amount.unwrap();
            self.held -= amount;
            self.total -= amount;
            self.locked = true;
        } else {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::InvalidIdReferenced,
                transaction: transaction,
                client: self,
            });
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// Log the transaction alongside any related transactions.
    fn log_transaction(&mut self, transaction: Transaction) {
        if let Some(related_transactions) = self.transactions.get_mut(&transaction.id) {
            related_transactions.push(transaction);
        } else {
            self.transactions.insert(transaction.id, vec![transaction]);
        }
    }
}

/// When serializing funds, attempt to round to four decimal places.
fn four_decimal_serializer<S>(n: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64((n * 1000.0).round() / 1000.0)
}
