#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]

//! transaction-manager is a simple utility to read a list of transactions and output the state of client accounts after all transactions are applied.
//!
//! ## Example Usage
//!
//! Will output account list to stdout:
//! <pre>
//! cargo run resources/transaction-list.csv
//! </pre>
//!
//! Will output account list to csv file:
//! <pre>
//! cargo run resources/transaction-list.csv > resources/account-list.csv
//! </pre>
//!
//! ## License
//!
//! This project is licensed under the MIT License - see the [LICENSE.md](https://github.com/tjhaskel/transaction-manager/blob/master/LICENSE.md) file for details

/// Represents a client account with id, amounts, and status
pub mod client;

/// Represents a client transaction with id, type, client id, and amount
pub mod transaction;

/// Represents various errors that could come from improper transactions
pub mod transaction_error;

/// Controller module that performs business logic based on input transactions and modifies client accounts accordingly.
pub mod transaction_manager;
