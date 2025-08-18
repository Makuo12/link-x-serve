use std::sync::RwLock;

use uuid::Uuid;

use crate::db_store::Store;


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
}


impl Cache {
    pub async fn new(store: &Store) -> Self {
        let devices = store.get_devices().await.unwrap_or_else(|_| vec![]);
        let customers = store.get_customers().await.unwrap_or_else(|_| vec![]);
        let accounts = store.get_accounts().await.unwrap_or_else(|_| vec![]);
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
        }
    }    
}