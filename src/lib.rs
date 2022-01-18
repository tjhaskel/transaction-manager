//#![warn(missing_docs)]
//#![warn(missing_doc_code_examples)]

/// Represents a client account with id, amounts, and status
pub mod client;

/// Represents a client transaction with id, type, client id, and amount
pub mod transaction;

/// Represents various errors that could come from improper transactions
pub mod transaction_error;

/// Controller module that performs business logic based on input transactions and modifies client accounts accordingly.
pub mod transaction_manager;

#[cfg(test)]
mod tests;
