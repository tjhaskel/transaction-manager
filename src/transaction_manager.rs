use csv::{ReaderBuilder, Trim, Writer};
use std::collections::HashMap;
use std::error::Error;

use crate::client::*;
use crate::transaction::*;
use crate::transaction_error::*;

pub fn process_transactions(
    transactions_file_path: &str,
    accounts_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut clients: HashMap<u16, Client> = HashMap::new();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(transactions_file_path);
    for next_transaction_result in reader?.deserialize() {
        let transaction: Transaction = next_transaction_result?;
        update_client(&mut clients, transaction)?;
    }
    write_accounts(clients, accounts_file_path)?;
    Ok(())
}

fn write_accounts(
    clients: HashMap<u16, Client>,
    accounts_file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_path(accounts_file_path)?;
    for (_, client) in clients {
        writer.serialize(client)?;
    }
    writer.flush()?;
    Ok(())
}

fn update_client(
    clients: &mut HashMap<u16, Client>,
    transaction: Transaction,
) -> Result<(), TransactionError> {
    let updated_client: Client = match clients.get(&transaction.client_id) {
        Some(client) => client.clone().apply_transaction(transaction)?,
        None => initialize_client(transaction)?,
    };
    clients.insert(updated_client.id, updated_client);
    Ok(())
}
