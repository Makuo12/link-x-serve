// use std::{env, sync::Arc, thread, time::Duration};

// use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::{IntoResponse, Response}, Json};
// use encrypt::{handle_decipher_device_id, handle_decipher_price, handle_env_bytes};
// use tokio::sync::RwLock;
// use tracing::info;

// use crate::{tools::constant::{CONNECT_MSG, ENCRYPTION_CONNECT_KEY, ENCRYPTION_DEVICE_ID_KEY, ENCRYPTION_KEY_PRICE, XMINISTER_API_KEY}, types::{api_key::ApiKey, pocket::{PocketConnectMsgResponse, PocketRequest, PocketResponse, TestResponse}}};



// pub async fn handle_device_pocket(State(account_store): State<Arc<RwLock<AccountStore>>>, Json(pocket): Json<PocketRequest>) -> Result<impl IntoResponse, Response> {
//     let price_key = env::var(ENCRYPTION_KEY_PRICE)
//     .map_err(|e| AppError::EnvError(e).into_response())?;
//     let device_id_key = env::var(ENCRYPTION_DEVICE_ID_KEY)
//     .map_err(|e| AppError::EnvError(e).into_response())?;
//     if pocket.price.len() < 16 || pocket.data.len() < 16 {
//         return Ok(AppError::DeviceNotFound.into_response());
//     }
//     let price = handle_decipher_price(
//         handle_env_bytes(price_key)
//         , pocket.price);
//     let device_id = handle_decipher_device_id(
//         handle_env_bytes(device_id_key), pocket.data)
//     .map_err(|e| AppError::AcmError(e).into_response())?;
//     {
//         let store = account_store.read().await;
//         let account = match store.get_account_res(&device_id) {
//             Some(res) => res,
//             None => return Ok(AppError::DeviceNotFound.into_response()),
//         };
//         Ok((StatusCode::OK, axum::Json(PocketResponse{price, account, currency: "NGN".to_string()})).into_response())
//     }
// }

// pub async fn payment_route() -> Result<impl IntoResponse, Response> {
//     thread::sleep(Duration::from_secs(6));
//     Ok((StatusCode::OK, axum::Json(TestResponse{success: true})).into_response())
// }

