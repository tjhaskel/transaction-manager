use std::error::Error;
use std::fmt;

use crate::client::*;
use crate::transaction::*;

#[derive(Debug)]
pub enum TransactionErrorTypes {
    NonPositiveAmount,
    MissingRequiredAmount,
    HasMeaninglessAmount,
    InsufficientFunds,
    InvalidIdReferenced,
    FirstTransactionNotDeposit,
    AccountLocked,
    Unspecified,
}

#[derive(Debug)]
pub struct TransactionError {
    pub error_type: TransactionErrorTypes,
    pub transaction: Transaction,
    pub client: Client,
}

impl fmt::Display for TransactionError {
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
