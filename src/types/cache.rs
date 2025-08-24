use std::sync::RwLock;

use uuid::Uuid;

use crate::{db_store::Store, types::device::Device};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BankFileStream {
    #[serde(rename = "SN")]
    pub serial_number: u32,
    #[serde(rename = "NAME OF INSTITUTION")]
    pub name: String,
    #[serde(rename = "HEAD OFFICE ADDRESS")]
    pub address: String,
    #[serde(rename = "STATE")]
    pub state: String,
}
pub fn get_nigerian_banks_from_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<BankFileStream>, Box<dyn std::error::Error>> {
    let json_content = fs::read_to_string(file_path)?;
    let banks: Vec<BankFileStream> = serde_json::from_str(&json_content)?;
    Ok(banks)
}

pub fn get_nigerian_banks() -> Result<Vec<BankFileStream>, Box<dyn std::error::Error>> {
    // Default to looking for "banks.json" in current directory
    get_nigerian_banks_from_file("banks.json")
}

// Helper functions for filtering and searching
impl BankFileStream {
    pub fn is_in_state(&self, state: &str) -> bool {
        self.state.eq_ignore_ascii_case(state)
    }
    pub fn get_bank_name(&self) -> &str {
        &self.name
    }
    pub fn contains_name(&self, search_term: &str) -> bool {
        self.name.to_lowercase().contains(&search_term.to_lowercase())
    }
}

pub struct DeviceStream {
    pub device_id: String,
    pub apk_key: String,
    pub id: Uuid,
    pub business_id: Uuid,
}
pub struct BankStream {
    pub id: String,
    pub user_id: Uuid,
    pub apk_key: String, // new field
}
pub struct CustomerStream {
    pub public_key: String,
    pub private_key: String,
    pub bank_id: String,
    pub id: Uuid
}

pub struct AccountStream {
    pub id: Uuid,
    pub bank_id: String,        // varchar → String
    pub account_name: String,
    pub account_number: String,
}

pub struct Cache {
    pub devices: RwLock<Vec<DeviceStream>>,
    pub customers: RwLock<Vec<CustomerStream>>,
    pub accounts: RwLock<Vec<AccountStream>>,
    pub banks: RwLock<Vec<BankStream>>,
    pub bank_files: RwLock<Vec<BankFileStream>>,
}


impl Cache {
    pub async fn new(store: &Store) -> Self {
        let bank_files = get_nigerian_banks().unwrap();
        let devices = store.get_devices().await.unwrap_or_else(|_| vec![]);
        let customers = store.get_customers().await.unwrap_or_else(|_| vec![]);
        let accounts = store.get_accounts().await.unwrap_or_else(|_| vec![]);
        // let mut main_device: Vec<Device> = Vec::new();
        // for device in devices.into_iter() {
        //     let id = device.main_id.clone();
        //     if main_device.iter().count() == 0 {
        //         main_device.push(device);
        //         continue;
        //     } else {
        //         let mut found = false;
        //         for main in main_device.iter() {
        //             if main.id == device.id {
        //                 found = true;
        //                 continue;
        //             }
        //         }
        //         if found {
        //             let _ = store.delete_device(id).await;
        //         } else {
        //             main_device.push(device);
        //         }
        //     }
        // }
        
        Cache {
            devices: RwLock::new(devices.iter().map(
                |f| DeviceStream {
                    device_id: f.device_id.clone(),
                    apk_key: f.apk_key.clone(),
                    id: f.id.clone(),
                    business_id: f.business_id.clone()
                }).collect()),
            customers: RwLock::new(customers.iter().map(
                |f| CustomerStream {
                    public_key: f.public_key.clone(),
                    private_key: f.private_key.clone(),
                    bank_id: f.bank_id.clone(),
                    id: f.id.clone()
                }
            ).collect()),
            accounts: RwLock::new(accounts.iter().map(
                |f| AccountStream {
                    id: f.id.clone(),
                    bank_id: f.bank_id.clone(),
                    account_name: f.account_name.clone(),
                    account_number: f.account_number.clone()
                }
            ).collect()),
            banks: RwLock::new(store.get_banks().await.unwrap_or_else(|_| vec![]).iter().map(
                |f| BankStream {
                    id: f.id.clone(),
                    user_id: f.user_id.clone(),
                    apk_key: f.apk_key.clone()
                }
            ).collect()),
            bank_files: RwLock::new(bank_files),
        }
    }    
}