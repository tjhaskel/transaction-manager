use crate::client::*;
use crate::transaction_io::*;

#[derive(Debug)]
pub enum TransactionType {
    Deposit,
    Withdrawel,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug)]
pub struct Transaction {
    pub id: u32,
    pub transaction_type: TransactionType,
    pub client_id: u16,
    pub amount: Option<f64>,
}
