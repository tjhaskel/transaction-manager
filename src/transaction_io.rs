use serde::{Deserialize, Serialize};
use std::io::prelude::*;

use crate::transaction::*;

pub fn initialize_stream(transaction_file_name: &str) -> () {
    println!("{}", transaction_file_name);
}

pub fn get_next_transaction() -> Transaction {
    Transaction {
        id: 0,
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        amount: 1.2,
    }
}