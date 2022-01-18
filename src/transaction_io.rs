use serde::{Deserialize, Serialize};
use std::io::prelude::*;

use crate::transaction::*;



pub fn initialize_reader(transaction_file_name: &str) -> () {
    println!("{}", transaction_file_name);
}

pub fn get_next_transaction() -> Option<Transaction> {
    Some(Transaction {
        id: 0,
        transaction_type: TransactionType::Deposit,
        client_id: 0,
        amount: Some(1.2),
    })
}
