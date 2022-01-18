use transaction_manager::transaction_manager::*;

fn main() {
    let transaction_file_path = "resources/transactions.csv";
    let accounts_file_path = "resources/accounts.csv";

    if let Err(e) = process_transactions(transaction_file_path, accounts_file_path) {
        println!(
            "Could not process transactions from {} to {}:\n\t{}",
            transaction_file_path, accounts_file_path, e
        );
    }
}
