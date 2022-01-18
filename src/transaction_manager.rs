use crate::client::*;
use crate::client_io::*;
use crate::transaction::*;
use crate::transaction_io::*;

pub fn process_transactions(transactions_file_path: &str) -> () {
    let mut clients: Vec<Client> = Vec::new();

    initialize_reader(transactions_file_path);
    println!("Processing transactions!");
    if let Some(next_transaction) = get_next_transaction() {
        update_clients(&mut clients, next_transaction);
    }
    println!("Writing account info to csv!");
    write_accounts(clients);
    println!("Done!");
}

pub fn write_accounts(clients: Vec<Client>) {
    for client in clients {
        println!("Client: {:#?}", client);
    }
}

fn update_clients(clients: &mut Vec<Client>, transaction: Transaction) {
    println!("Transaction: {:#?}", transaction);
}
