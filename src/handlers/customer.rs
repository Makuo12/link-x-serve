use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Response}, Extension, Json};
use encrypt::ecc::generate_keys;
use handle_error::Error;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::{db_store::Store, handlers::middleware::{AuthenticatedApk, AuthenticatedUser}, types::cache::Cache};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerRequest {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomerResponse {
    public_key: String,
    public_id: uuid::Uuid,
    msg: uuid::Uuid // MSG would just be the main ID
}


pub async fn create_customer(State(state): State<(Store, Arc<Cache>)>, Extension(user): Extension<AuthenticatedApk>,Json(packet): Json<CustomerRequest>) ->Result<impl IntoResponse, Response> {
    let store = state.0;
    let cache = state.1;
    let mut found = false;
    let mut bank_id: String = String::new();
    if let Ok(banks) = cache.banks.try_read() {
        for bank in banks.iter() {
            if bank.apk_key == user.apk {
                bank_id = bank.id.clone();
                found = true;
                break;
            }
        }
    }
    if !found {
        info!("not found");
        return Ok(Error::Unauthorized.into_response());
    }
    let keys = generate_keys().map_err(|e| e.into_response())?;
    let result = store.add_customer(packet.first_name, packet.last_name, keys.public_key, keys.private_key, &bank_id, &keys.file_name, uuid::Uuid::new_v4()).await.map_err(|e| e.into_response())?;
    let customer = store.get_customer(result).await.map_err(|e| e.into_response())?;
    Ok(Json(CustomerResponse {
        public_key: customer.public_key,
        msg: customer.id,
        public_id: customer.public_id,
    }).into_response())
}

