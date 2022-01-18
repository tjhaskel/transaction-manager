use std::error::Error;
use std::fmt;

use crate::client::*;
use crate::transaction::*;

/// Represents various errors that could be encountered when attempting to apply transactions incorrectly.
#[derive(Debug)]
pub enum TransactionErrorTypes {
    /// If deposit or withdrawal are attempted with zero or negative amount.
    NonPositiveAmount,
    /// If deposit or withdrawal are attempted with no amount specified.
    MissingRequiredAmount,
    /// If dispute, resolve, or chargeback are attempted with an amount specified.
    HasMeaninglessAmount,
    /// If withdrawal is atempted with amount greater than available funds.
    InsufficientFunds,
    /// If dispute, resolve, or chargeback are attempted with invalid transaction id referenced.
    InvalidIdReferenced,
    /// If a client is initialized with any transaction type other tha deposit.
    FirstTransactionNotDeposit,
    /// If any transaction is attempted on a locked account.
    AccountLocked,
    /// Generic error for very uncommon issues.
    Unspecified,
}

/// A transaction error has a type and captures the transaction/account environment in which it occured.
#[derive(Debug)]
pub struct TransactionError {
    /// One of the specified error types, or unspecified.
    pub error_type: TransactionErrorTypes,
    /// The transaction that caused the error.
    pub transaction: Transaction,
    /// The client account that the transaction was attempted on.
    pub client: Client,
}

impl fmt::Display for TransactionError {
    /// Readable error messages for each specified type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self.error_type {
            TransactionErrorTypes::NonPositiveAmount => {
                "Negative or zero value provided for transaction amount."
            }
            TransactionErrorTypes::MissingRequiredAmount => {
                "Deposit or withdrawel without specified amount."
            }
            TransactionErrorTypes::HasMeaninglessAmount => {
                "Dispute, resolve, or chargeback with specified amount."
            }
            TransactionErrorTypes::InsufficientFunds => "Insufficient funds for transaction.",
            TransactionErrorTypes::FirstTransactionNotDeposit => {
                "First transaction is not deposit."
            }
            TransactionErrorTypes::InvalidIdReferenced => {
                "Dispute, resolve, or chargeback with invalid id."
            }
            TransactionErrorTypes::AccountLocked => {
                "Attempted to apply transaction to locked account."
            }
            TransactionErrorTypes::Unspecified => "Unspecified.",
        };
        write!(
            f,
            "Error: {}\nTransaction: {:#?}\nClient: {:#?}",
            message, self.transaction, self.client
        )
    }
}

impl Error for TransactionError {}
