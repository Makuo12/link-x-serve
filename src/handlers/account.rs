use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use serde::{Deserialize, Serialize};

use crate::{db_store::Store, handlers::middleware::AuthenticatedUser, types::cache::Cache};
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserAccountResponse {
    list: Vec<AccountResponse>
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccountResponse {
    bank_name: String,
    account_name: String,
    account_number: String,
    bank_id: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn get_account(
    State(state): State<(Store, Arc<Cache>)>, 
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, Response> {
    let store = state.0;
    let cache = state.1;
    // Get user by ID from the authenticated token
    let result_user = store.get_user(user.user_id).await;
    let user_data = match result_user {
        Ok(u) => u,
        Err(e) => return Ok(e.into_response()),
    };
    let businesses = match store.get_businesses_user_id(user_data.id).await {
        Ok(b) => b,
        Err(e) => return Ok(e.into_response())
    };
    let mut account_response: Vec<AccountResponse> = Vec::new();
    for b in businesses {
        let devices = match store.get_devices_by_business_id(&b.id).await {
            Ok(b) => b,
            Err(e) => return Ok(e.into_response())
        };
        for device in devices {
            let account = match store.get_account(&device.account_id).await {
                Ok(b) => b,
                Err(e) => return Ok(e.into_response())
            };
            let mut found = false;
            for res in account_response.iter() {
                if *res.bank_id == account.bank_id {
                    found = true;
                    break;
                }
            }
            if found {
                continue;
            }
            let bank_file_result = cache.bank_files.try_read();
            match bank_file_result {
                Ok(bank_file) => {
                    let mut bank_name = String::new();
                    for file in bank_file.iter() {
                        if file.serial_number.to_string() == account.bank_id {
                            bank_name = file.name.to_string();
                        }
                    }
                    account_response.push(AccountResponse {
                        bank_name: bank_name,
                        account_name: account.account_name,
                        account_number: account.account_number,
                        bank_id: account.bank_id.to_string(),
                        created_at: account.created_at,
                        updated_at: account.updated_at
                    });
                },
                Err(_) => continue
            }
        }
    }
    return Ok((StatusCode::OK, Json(UserAccountResponse{list: account_response})).into_response());
}