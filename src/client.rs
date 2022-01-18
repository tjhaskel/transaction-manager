use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
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
    pub available: f64,

    /// Funds held in dispute.
    #[serde(serialize_with = "four_decimal_serializer")]
    pub held: f64,

    /// Total funds in account.
    #[serde(serialize_with = "four_decimal_serializer")]
    pub total: f64,

    /// Locked is true if a chargeback has been issued.
    pub locked: bool,

    /// A log of prevous transactions, grouped by transaction ID.
    #[serde(skip_serializing)]
    pub transactions: BTreeMap<u32, Vec<Transaction>>,
}

/// Create a new client with default settings, then apply their first transaction.
/// ```
/// use transaction_manager::transaction::*;
/// use transaction_manager::client::*;
/// let client = initialize_client(Transaction {
///     transaction_type: TransactionType::Deposit,
///     client_id: 0,
///     id: 0,
///     amount: Some(1.2)
/// }).unwrap();
/// assert_eq!(client.id, 0);
/// assert_eq!(client.available, 1.2);
/// assert_eq!(client.held, 0.0);
/// assert_eq!(client.total, 1.2);
/// assert_eq!(client.locked, false);
/// assert_eq!(client.transactions[&0][0].amount, Some(1.2));
/// ```
pub fn initialize_client(transaction: Transaction) -> Result<Client, TransactionError> {
    let client = Client {
        id: transaction.client_id,
        available: 0.0,
        held: 0.0,
        total: 0.0,
        locked: false,
        transactions: BTreeMap::new(),
    };
    if transaction.transaction_type != TransactionType::Deposit {
        return Err(TransactionError {
            error_type: TransactionErrorTypes::FirstTransactionNotDeposit,
            transaction: transaction,
            client: client,
        });
    }
    Ok(client.apply_transaction(transaction)?)
}

impl Client {
    /// Try to apply the given tranasaction to the client, and if successful return the updated client.
    /// May produce a TransactionError if the transaction breaks any rules.
    /// ```
    /// use transaction_manager::transaction::*;
    /// use transaction_manager::client::*;
    /// let client = initialize_client(Transaction {
    ///     transaction_type: TransactionType::Deposit,
    ///     client_id: 0,
    ///     id: 0,
    ///     amount: Some(1.2)
    /// }).unwrap();
    ///
    /// let client = client.apply_transaction(Transaction {
    /// transaction_type: TransactionType::Deposit,
    /// client_id: 0,
    /// id: 1,
    /// amount: Some(1.3)}).unwrap();
    /// assert_eq!(client.id, 0);
    /// assert_eq!(client.available, 2.5);
    /// assert_eq!(client.held, 0.0);
    /// assert_eq!(client.total, 2.5);
    /// assert_eq!(client.locked, false);
    /// assert_eq!(client.transactions[&1][0].amount, Some(1.3));
    /// ```
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
            self.available = round_to_four_decimals(self.available + amount);
            self.total = round_to_four_decimals(self.total + amount);
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
            self.available = round_to_four_decimals(self.available - amount);
            self.total = round_to_four_decimals(self.total - amount);
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
    /// If the referenced transaction ID does not exist, ignore and log the the transaction.
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
            self.available = round_to_four_decimals(self.available - amount);
            self.held = round_to_four_decimals(self.held + amount);
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given transaction ID exists in the log and a dispute was the last transaction, move the amount specified in that deposit from held to available.
    /// If the referenced transaction ID does not exist or does not reference a dispute, ignore and log the the transaction.
    fn apply_resolve(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(_) = transaction.amount {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::HasMeaninglessAmount,
                transaction: transaction,
                client: self,
            });
        }
        if let Some(related_transactions) = self.transactions.get(&transaction.id) {
            if related_transactions[related_transactions.len() - 1].transaction_type
                == TransactionType::Dispute
            {
                let amount = related_transactions[0].amount.unwrap();
                self.held = round_to_four_decimals(self.held - amount);
                self.available = round_to_four_decimals(self.available + amount);
            }
        }
        self.log_transaction(transaction);
        Ok(self)
    }

    /// If the given transaction ID exists in the log and a dispute was the last transaction, subrtract the amount specified in that deposit from held and total, then lock the account.
    /// If the referenced transaction ID does not exist or does not reference a dispute, ignore and log the the transaction
    fn apply_chargeback(mut self, transaction: Transaction) -> Result<Client, TransactionError> {
        if let Some(_) = transaction.amount {
            return Err(TransactionError {
                error_type: TransactionErrorTypes::HasMeaninglessAmount,
                transaction: transaction,
                client: self,
            });
        }
        if let Some(related_transactions) = self.transactions.get(&transaction.id) {
            if related_transactions[related_transactions.len() - 1].transaction_type
                == TransactionType::Dispute
            {
                let amount = related_transactions[0].amount.unwrap();
                self.held = round_to_four_decimals(self.held - amount);
                self.total = round_to_four_decimals(self.total - amount);
                self.locked = true;
            }
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

/// Rounds any f64 to four decimal places
fn round_to_four_decimals(n: f64) -> f64 {
    (n * 10000.0).round() / 10000.0
}

/// When serializing funds, attempt to round to four decimal places.
fn four_decimal_serializer<S>(n: &f64, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64(round_to_four_decimals(*n))
}

#[test]
fn test_deposit() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let client = client
        .apply_deposit(Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 0,
            id: 1,
            amount: Some(1.3),
        })
        .unwrap();
    assert_eq!(client.available, 2.5);
    assert_eq!(client.held, 0.0);
    assert_eq!(client.total, 2.5);
    assert_eq!(client.locked, false);
}

#[test]
fn test_withdrawal() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let client = client
        .apply_withdrawal(Transaction {
            transaction_type: TransactionType::Withdrawal,
            client_id: 0,
            id: 1,
            amount: Some(1.1),
        })
        .unwrap();
    assert_eq!(client.available, 0.1);
    assert_eq!(client.held, 0.0);
    assert_eq!(client.total, 0.1);
    assert_eq!(client.locked, false);
}

#[test]
fn test_dispute() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let client = client
        .apply_dispute(Transaction {
            transaction_type: TransactionType::Dispute,
            client_id: 0,
            id: 0,
            amount: None,
        })
        .unwrap();
    assert_eq!(client.available, 0.0);
    assert_eq!(client.held, 1.2);
    assert_eq!(client.total, 1.2);
    assert_eq!(client.locked, false);
}

#[test]
fn test_resolve() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let client = client
        .apply_dispute(Transaction {
            transaction_type: TransactionType::Dispute,
            client_id: 0,
            id: 0,
            amount: None,
        })
        .unwrap();
    let client = client
        .apply_resolve(Transaction {
            transaction_type: TransactionType::Resolve,
            client_id: 0,
            id: 0,
            amount: None,
        })
        .unwrap();
    assert_eq!(client.available, 1.2);
    assert_eq!(client.held, 0.0);
    assert_eq!(client.total, 1.2);
    assert_eq!(client.locked, false);
}

#[test]
fn test_chargeback() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let client = client
        .apply_dispute(Transaction {
            transaction_type: TransactionType::Dispute,
            client_id: 0,
            id: 0,
            amount: None,
        })
        .unwrap();
    let client = client
        .apply_chargeback(Transaction {
            transaction_type: TransactionType::Chargeback,
            client_id: 0,
            id: 0,
            amount: None,
        })
        .unwrap();
    assert_eq!(client.available, 0.0);
    assert_eq!(client.held, 0.0);
    assert_eq!(client.total, 0.0);
    assert_eq!(client.locked, true);
}

#[test]
fn test_log_transaction() {
    let client = initialize_client(Transaction {
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        id: 0,
        amount: Some(1.2),
    })
    .unwrap();
    let transaction_one = Transaction {
        transaction_type: TransactionType::Dispute,
        client_id: 0,
        id: 0,
        amount: None,
    };
    let transaction_two = transaction_one.clone();
    let client = client.apply_dispute(transaction_one).unwrap();
    assert_eq!(client.transactions[&0][1], transaction_two);
}

#[test]
fn test_round_to_four_decimals() {
    assert_eq!(round_to_four_decimals(0.1234001), 0.1234);
}
