use csv::{ReaderBuilder, Trim, Writer};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::Write;

use crate::client::*;
use crate::transaction::*;
use crate::transaction_error::*;

/// Reads from the given transaction csv file path, applying each transaction one at a time to the client account environment.
/// Once all transactions have been processed, the client account environment is serialized and written to stdout.
/// May produce an error if reading, serializing, or writing fails, or if there is any invalid transaction.
/// ```
/// use transaction_manager::transaction_manager::*;
///
/// let mut output = Vec::new();
/// process_transactions(&mut output, "resources/transaction-list.csv").unwrap();
/// let output = String::from_utf8(output).expect("Not UTF-8");
///
/// assert_eq!(output, "\
/// client,available,held,total,locked
/// 1,1.0,0.0,1.0,false
/// 2,0.0,3.3,3.3,false
/// 3,4.0,0.0,4.0,false
/// 4,5.0,0.0,5.0,true
/// ");
/// ```
pub fn process_transactions<W>(
    writer: W,
    transactions_file_path: &str,
) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    let mut clients: BTreeMap<u16, Client> = BTreeMap::new();
    let reader = ReaderBuilder::new()
        .trim(Trim::All)
        .from_path(transactions_file_path);
    for next_transaction_result in reader?.deserialize() {
        let transaction: Transaction = next_transaction_result?;
        update_client(&mut clients, transaction)?;
    }
    write_accounts(writer, clients)?;
    Ok(())
}

/// Attempt to apply the given transaction to the given client account environment.
/// May produce a TransactionError if any rules are violated.
fn update_client(
    clients: &mut BTreeMap<u16, Client>,
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
fn write_accounts<W>(writer: W, clients: BTreeMap<u16, Client>) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    let mut writer = Writer::from_writer(writer);
    for (_, client) in clients {
        writer.serialize(client)?;
    }
    writer.flush()?;
    Ok(())
}

#[test]
fn test_update_client() -> () {
    let mut clients: BTreeMap<u16, Client> = BTreeMap::new();
    update_client(
        &mut clients,
        Transaction {
            transaction_type: TransactionType::Deposit,
            client_id: 0,
            id: 0,
            amount: Some(1.2),
        },
    )
    .unwrap();
    assert_eq!(clients.len(), 1);
    let client = clients[&0].clone();
    assert_eq!(client.id, 0);
    assert_eq!(client.available, 1.2);
    assert_eq!(client.held, 0.0);
    assert_eq!(client.total, 1.2);
    assert_eq!(client.locked, false);
    assert_eq!(client.locked, false);
    assert_eq!(client.transactions[&0].len(), 1);
}

#[test]
fn test_write_accounts() -> () {
    let mut clients: BTreeMap<u16, Client> = BTreeMap::new();
    clients.insert(
        0,
        Client {
            id: 1,
            available: 1.0,
            held: 0.0,
            total: 1.0,
            locked: false,
            transactions: BTreeMap::new(),
        },
    );
    let mut output = Vec::new();
    write_accounts(&mut output, clients).unwrap();
    let output = String::from_utf8(output).expect("Not UTF-8");
    assert_eq!(
        output,
        "\
client,available,held,total,locked
1,1.0,0.0,1.0,false
"
    );
}
