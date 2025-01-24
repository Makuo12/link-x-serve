use serde::{Deserialize, Serialize};

use super::account::AccountResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct PocketRequest {
    pub data: Vec<u8>,
    pub price: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PocketConnectMsgResponse {
    pub msg: [u8; 16]
}

#[derive(Serialize, Deserialize)]
pub struct PocketResponse {
    pub price: u64,
    pub account: AccountResponse,
    pub currency: String,
}