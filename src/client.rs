use std::collections::HashMap;
use std::vec::Vec;

use crate::client_io::*;
use crate::transaction::*;

#[derive(Debug)]
pub struct Client {
    id: u16,
    available: f64,
    held: f64,
    total: f64,
    locked: bool,
    transactions: HashMap<u32, Vec<Transaction>>
}
