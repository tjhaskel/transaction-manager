use std::collections::HashMap;
use std::vec::Vec;

use crate::client_io::*;
use crate::transaction::*;

#[derive(Debug)]
pub struct Client {
    pub id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
    pub transactions: HashMap<u32, Vec<Transaction>>
}
