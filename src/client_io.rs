use serde::{Deserialize, Serialize};
use std::io::prelude::*;

use crate::client::*;

pub fn initialize_stream(accounts_file_name: &str) -> () {
    println!("{}", accounts_file_name);
}

pub fn write_client_info(client: Client) -> () {
    println!("{:#?}", client);
}
