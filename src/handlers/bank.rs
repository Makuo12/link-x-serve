use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db_store::Store, handlers::middleware::AuthenticatedUser, types::cache::Cache};
use handle_error::Error;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BankResponse {
    pub id: String,
    pub user_id: Uuid,
    pub apk_key: String, // new field
    pub bank_name: String,
    pub location: String,
    pub state: String,
}


pub async fn get_bank(
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
    let bank = match store.get_bank_user_id(&user_data.id).await {
        Ok(b) => b,
        Err(e) => return Ok(e.into_response())
    };
    let mut bank_response: Option<BankResponse> = Option::None;
    match cache.bank_files.try_read() {
        Ok(banks) => {
            for b in banks.iter() {
                if *b.serial_number.to_string() == bank.id {
                    bank_response = Some(BankResponse{
                        id: bank.id.to_string(),
                        user_id: bank.user_id,
                        apk_key: bank.apk_key.to_string(),
                        bank_name: b.name.to_string(),
                        location: b.address.to_string(),
                        state: b.state.to_string()
                    })
                }
            }
        },
        Err(_) => {return Ok(Error::Unauthorized.into_response())}
    };
    if let Some(response) = bank_response {
        return Ok((StatusCode::OK, Json(
            response
        )).into_response());
    }
    return Ok(Error::Unauthorized.into_response())
}