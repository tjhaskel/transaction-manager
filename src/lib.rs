//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]

/// Represents a client account with id, amounts, and status
pub mod client;

/// Writes client account info to a csv file
pub mod client_io;

/// Represents a client transaction with id, type, client id, and amount
pub mod transaction;

/// Reads from a csv file containing transactions
pub mod transaction_io;

/// Controller module that performs business logic based on input transactions and modifies client accounts accordingly. 
pub mod transaction_manager;

#[cfg(test)]
mod tests;
