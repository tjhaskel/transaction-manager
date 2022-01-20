use std::env;
use std::io;

use transaction_manager::transaction_manager::*;

/// This program should be called with a single argument representing a csv file with transaction data. See resources/transaction-list.csv for an example.
/// It outputs a list of accounts to stdout, which in turn can be piped to a csv file. See resources/account-list.csv for an example.
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let transaction_file_path = &args[1];
        process_transactions(io::stdout(), transaction_file_path).unwrap()
    }
}
