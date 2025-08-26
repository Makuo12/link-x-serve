
use std::{str::FromStr, sync::Arc};

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use chrono::Utc;
use encrypt::{ecc::{ecc_decrypt_key, generate_keys}, functions::decrypt};
use handle_error::Error;
use crate::{db_store::Store, handlers::middleware::{AuthenticatedApk, AuthenticatedUser}, types::{cache::Cache, payments::PaymentResponse}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetalPaymentRequest {
    customer_id: uuid::Uuid,
    device_id: String,
    pipe: String,
    encrypted_price: Vec<u8>,
    time: i64
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MetalPaymentResponse {
    first_name: String,
    last_name: String,
    amount: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPaymentResponse {
    list: Vec<PaymentResponse>
}

pub async fn metal_pay(State(state): State<(Store, Arc<Cache>)>, Extension(metal): Extension<AuthenticatedApk>,Json(packet): Json<MetalPaymentRequest>) ->Result<impl IntoResponse, Response> {
    if (packet.time / 1000) > 8 {
        return Err(Error::ApiKeyRejection.into_response());
    }
    let store = state.0;
    // let cache = state.1;
    let customer = store.get_customer_public_id(packet.customer_id).await.map_err(|e| e.into_response())?;
    let encrypt_msg = ecc_decrypt_key(&packet.pipe, customer.private_key).map_err(|e| e.into_response())?;
    let encrypt_msg_split: Vec<&str> = encrypt_msg.split('&').collect();
    if encrypt_msg_split.len() != 2 {
        return Err(Error::Unauthorized.into_response());
    }
    let customer_main_id = uuid::Uuid::from_str(&encrypt_msg_split[0]).map_err(|e| Error::ApiKeyRejection.into_response())?;
    if customer_main_id != customer.id {
        return Err(Error::Unauthorized.into_response());
    }
    let data = store.get_device_with_business_user_account(packet.device_id).await.map_err(|e| e.into_response())?;
    let decrypt_price = decrypt(&packet.encrypted_price, &data.price_key.as_bytes()).map_err(|e| Error::AcmError(e).into_response())?;
    let mut sum: i64 = 0;
    for c in decrypt_price.iter() {
        if c < &b'0' || c > &b'9' {
            // 97 represent a
            if *c == 97 {
                break;
            } else {
                return Err(Error::Unauthorized.into_response());
            }
        }
        sum = (sum*10) + (c - b'0') as i64;
    }
    let result = store.add_payment(data.main_device_id, sum, customer.id, data.user_id, data.account_bank_id, data.account_name, data.account_number).await.map_err(|e| e.into_response())?;
    let response = MetalPaymentResponse {
        first_name: customer.first_name,
        last_name: customer.last_name,
        amount: result.amount,
    };
    Ok(Json(response).into_response())
}


pub async fn customer_pay(State(state): State<(Store, Arc<Cache>)>, Extension(metal): Extension<AuthenticatedApk>,Json(packet): Json<MetalPaymentRequest>) ->Result<impl IntoResponse, Response> {
    let store = state.0;
    // let cache = state.1;
    let customer = store.get_customer_public_id(packet.customer_id).await.map_err(|e| e.into_response())?;
    let encrypt_msg = ecc_decrypt_key(&packet.pipe, customer.private_key).map_err(|e| e.into_response())?;
    let encrypt_msg_split: Vec<&str> = encrypt_msg.split('&').collect();
    if encrypt_msg_split.len() != 2 {
        return Err(Error::Unauthorized.into_response());
    }
    let customer_main_id = uuid::Uuid::from_str(&encrypt_msg_split[0]).map_err(|e| Error::ApiKeyRejection.into_response())?;
    if customer_main_id != customer.id {
        return Err(Error::Unauthorized.into_response());
    }
    let time = &encrypt_msg_split[1].parse::<i64>().map_err(|_| Error::ApiKeyRejection.into_response())?;
    if (Utc::now().timestamp() - time) > 10 {
        return Err(Error::Unauthorized.into_response());
    }
    let data = store.get_device_with_business_user_account(packet.device_id).await.map_err(|e| e.into_response())?;
    let decrypt_price = decrypt(&packet.encrypted_price, &data.price_key.as_bytes()).map_err(|e| Error::AcmError(e).into_response())?;
    let mut sum: i64 = 0;
    for c in decrypt_price.iter() {
        if c < &b'0' || c > &b'9' {
            return Err(Error::Unauthorized.into_response());
        }
        sum = (sum*10) + (c - b'0') as i64;
    }
    let result = store.add_payment(data.main_device_id, sum, customer.id, data.user_id, data.account_bank_id, data.account_name, data.account_number).await.map_err(|e| e.into_response())?;
    Ok(Json(result).into_response())
}




pub async fn get_payments(
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
    let payments = match store.get_payments_user_id(user_data.id).await {
        Ok(b) => b,
        Err(e) => return Ok(e.into_response())
    };
    let response = UserPaymentResponse {
        list: payments
    };
    return Ok((StatusCode::OK, Json(response)).into_response());
}