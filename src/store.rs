use std::{collections::{HashMap, HashSet}, fs::{self, File}};

use bytes::Bytes;

use crate::types::account::{Account, AccountItemStore, AccountResponse};


pub struct AccountStore {
    pub accounts: HashMap<[u8; 16], Account>
}

impl AccountStore {
    pub fn new() -> Self {
        let mut accounts = HashMap::new();
        let contents = fs::read_to_string("accounts.json").unwrap();
        let data: AccountItemStore = serde_json::from_str(&contents).unwrap();
        for item in data.accounts {
            let mut id: [u8; 16] = [0; 16];
            for i in item.id.iter().enumerate() {
                id[i.0] = *i.1;
            }
            let mut account_number: [u8; 10] = [0; 10];
            for i in item.account_number.iter().enumerate() {
                account_number[i.0] = *i.1;
            }
            let account = Account {
                account_number,
                account_name: Bytes::from(item.account_name),
                bank_id: Bytes::from(item.bank_id),
                bank_name: Bytes::from(item.bank_name),
            };
            accounts.insert(id, account);
        }
        AccountStore {
            accounts
        }
    }
    pub fn get_account_res(&self, id: &[u8; 16]) -> Option<AccountResponse> {
        let account = self.accounts.get(id)?;
        let mut account_name = String::new();
        let mut bank_id = String::new();
        let mut bank_name = String::new();
        for b in account.account_name.as_ref() {
            account_name.push(*b as char);
        }
        for b in account.bank_id.as_ref() {
            bank_id.push(*b as char);
        }
        for b in account.bank_name.as_ref() {
            bank_name.push(*b as char);
        }
        Some(
            AccountResponse {
            account_number: account.account_number.to_vec(),
            account_name,
            bank_id,
            bank_name,
        }
        )
    }
}