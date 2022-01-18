use serde::Deserialize;

/// Represent the types of transactions accepted
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    /// Adding funds.
    Deposit,
    /// Removing funds.
    Withdrawal,
    /// Holding funds.
    Dispute,
    /// Releasing funds.
    Resolve,
    /// Removing funds forcefully. Locks account.
    Chargeback,
}

/// A transaction has a type, client id, transaction id, and optional amount.
#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    /// Transaction type
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,

    /// Unique client ID
    #[serde(rename = "client")]
    pub client_id: u16,

    /// Unique or referenced transaction ID
    #[serde(rename = "tx")]
    pub id: u32,

    /// Amount is specified only for deposit or withdrawal
    pub amount: Option<f64>,
}
