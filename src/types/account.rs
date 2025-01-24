use bytes::Bytes;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub account_number: Vec<u8>,
    pub account_name: String,
    pub bank_id: String,
    pub bank_name: String
}

pub struct Account {
    pub account_number:  [u8; 10],
    pub account_name: Bytes,
    pub bank_id: Bytes,
    pub bank_name: Bytes,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountItem {
    pub id: Vec<u8>,
    pub account_number: Vec<u8>,
    pub account_name: String,
    pub bank_id: String,
    pub bank_name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountItemStore {
    pub accounts: Vec<AccountItem>,
}