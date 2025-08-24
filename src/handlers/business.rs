

// Table devices as D {
//   id uuid [pk, ref: - DA.id, unique, not null]
//   device_id varchar [not null]
//   name varchar [not null]
//   account_id uuid [ref: > A.id, not null]
//   device_type varchar [not null]
//   apk_key varchar [not null]
//   business_id uuid [ref: > BU.id, not null]
//   created_at timestamptz [not null, default:`now()`]
//   updated_at timestamptz [not null, default: `now()`]
// }


use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use encrypt::generate_random_char;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db_store::Store, handlers::middleware::AuthenticatedUser, types::{business::GeoPoint, cache::Cache}};
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserBusinessResponse {
    list: Vec<BusinessResponse>
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct BusinessResponse {
    name: String,
    location: String,
    geolocation: GeoPoint,

    devices: Vec<DeviceResponse>
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct AccountDeviceResponse {
    bank_name: String,
    account_name: String,
    account_number: String,
    bank_id: String
}
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DeviceResponse {
    id: Uuid,
    device_id: String,
    name: String,
    device_type: String,
    account: AccountDeviceResponse,
    active: bool,
}

pub async fn get_business(
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
    let mut business_response: Vec<BusinessResponse> = Vec::new();
    for b in businesses {
        let devices = match store.get_devices_by_business_id(&b.id).await {
            Ok(b) => b,
            Err(e) => return Ok(e.into_response())
        };
        let mut device_response: Vec<DeviceResponse> = Vec::new();
        for device in devices {
            let account = match store.get_account(&device.account_id).await {
                Ok(b) => b,
                Err(e) => return Ok(e.into_response())
            };
            let bank_file_result = cache.bank_files.try_read();
            match bank_file_result {
                Ok(bank_file) => {
                    let mut bank_name = String::new();
                    for file in bank_file.iter() {
                        if file.serial_number.to_string() == account.bank_id {
                            bank_name = file.name.to_string();
                        }
                    }
                    let serial_number = account.bank_id.clone();
                    let response = DeviceResponse {
                        id: device.id,
                        device_id: device.device_id,
                        device_type: device.device_type,
                        name: device.name,
                        account: AccountDeviceResponse { bank_name: bank_name, account_name: account.account_name, account_number: account.account_number, bank_id: serial_number },
                        active: true,
                    };
                    device_response.push(response);
                },
                Err(_) => continue
            }
        }
        business_response.push(BusinessResponse { name: b.name, location: b.location, geolocation: GeoPoint { latitude: b.lat, longitude: b.long }, devices: device_response });
    }
    return Ok((StatusCode::OK, Json(UserBusinessResponse{list: business_response})).into_response());
}


pub async fn setup_device(store: &Store) {
    for device in store.get_devices().await.unwrap() {
        let device_id = generate_random_char(16);
        let price_key = generate_random_char(16);
        let id_key = generate_random_char(16);
        store.setup_device(device.id, device_id, price_key, id_key).await.unwrap();
    }
}