use std::env;

use transaction_manager::transaction_manager::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let transaction_file_path = &args[1];

        if let Err(e) = process_transactions(transaction_file_path) {
            println!(
                "Could not process transactions from {}:\n\t{}",
                transaction_file_path, e
            );
        }
    } else {
        println!("Please provide a transaction csv file input.");
    }
}
