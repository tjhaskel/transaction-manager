use csv::{ReaderBuilder, Trim, Writer};
use std::collections::HashMap;
use std::error::Error;
use std::io;

use crate::client::*;
use crate::transaction::*;
use crate::transaction_error::*;

/// Reads from the given transaction csv file path, applying each transaction one at a time to the client account environment.
/// Once all transactions have been processed, the client account environment is serialized and written to stdout.
/// May produce an error if reading, serializing, or writing fails, or if there is any invalid transaction.
pub fn process_transactions(transactions_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut clients: HashMap<u16, Client> = HashMap::new();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(transactions_file_path);
    for next_transaction_result in reader?.deserialize() {
        let transaction: Transaction = next_transaction_result?;
        update_client(&mut clients, transaction)?;
    }
    write_accounts(clients)?;
    Ok(())
}

/// Attempt to apply the given transaction to the given client account environment.
/// May produce a TransactionError if any rules are violated.
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

/// Serialize the given client account environment to csv format and write it to stdout
/// May produce an error if there is a problem serializing the data or writing.
fn write_accounts(clients: HashMap<u16, Client>) -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::from_writer(io::stdout());
    for (_, client) in clients {
        writer.serialize(client)?;
    }
    writer.flush()?;
    Ok(())
}
