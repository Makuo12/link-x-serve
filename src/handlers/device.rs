// use axum::{
//     extract::{Path, State},
//     http::StatusCode,
//     response::{IntoResponse, Response},
//     Json,
// };
// use chrono::Utc;
// use serde::{Deserialize, Serialize};
// use uuid::Uuid;

// use crate::{db_store::Store, types::device::Device};

// use super::middleware::AuthenticatedUser; // for authentication


// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct DevicePostRequest {
//     pub device_id: String,
//     pub name: String,
//     pub device_type: String,
//     pub apk_key: String,
//     pub business_id: Uuid,
//     pub account_id: Uuid,
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct DeviceResponse {
//     pub id: Uuid,
//     pub device_id: String,
//     pub name: String,
//     pub device_type: String,
//     pub apk_key: String,
//     pub business_id: Uuid,
//     pub account_id: Uuid,
//     pub created_at: chrono::DateTime<Utc>,
//     pub updated_at: chrono::DateTime<Utc>,
// }

// /// Register a new device
// pub async fn register_device(
//     State(store): State<(Store, Arc<Cache>)>,
//     Json(packet): Json<DevicePostRequest>,
// ) -> Result<impl IntoResponse, Response> {
//     let device = Device {
//         id: Uuid::new_v4(),
//         main_id: Uuid::new_v4(),
//         device_id: packet.device_id,
//         name: packet.name,
//         device_type: packet.device_type,
//         apk_key: packet.apk_key,
//         business_id: packet.business_id,
//         account_id: packet.account_id,
//         created_at: Utc::now(),
//         updated_at: Utc::now(),
//     };

//     match store.add_device(&device).await {
//         Ok(_) => {
//             let response = DeviceResponse {
//                 id: device.id,
//                 device_id: device.device_id,
//                 name: device.name,
//                 device_type: device.device_type,
//                 apk_key: device.apk_key,
//                 business_id: device.business_id,
//                 account_id: device.account_id,
//                 created_at: device.created_at,
//                 updated_at: device.updated_at,
//             };
//             Ok((StatusCode::CREATED, Json(response)).into_response())
//         }
//         Err(e) => Ok(e.into_response()),
//     }
// }

// /// Get one device by ID
// pub async fn get_device_profile(
//     State(store): State<(Store, Arc<Cache>)>,
//     Path(id): Path<Uuid>,
// ) -> Result<impl IntoResponse, Response> {
//     match store.get_device(id).await {
//         Ok(device) => {
//             let response = DeviceResponse {
//                 id: device.id,
//                 device_id: device.device_id,
//                 name: device.name,
//                 device_type: device.device_type,
//                 apk_key: device.apk_key,
//                 business_id: device.business_id,
//                 account_id: device.account_id,
//                 created_at: device.created_at,
//                 updated_at: device.updated_at,
//             };
//             Ok((StatusCode::OK, Json(response)).into_response())
//         }
//         Err(e) => Ok(e.into_response()),
//     }
// }

// /// Get all devices
// pub async fn get_all_devices(
//     State(store): State<(Store, Arc<Cache>)>,
// ) -> Result<impl IntoResponse, Response> {
//     match store.get_devices().await {
//         Ok(devices) => {
//             let response: Vec<DeviceResponse> = devices
//                 .into_iter()
//                 .map(|d| DeviceResponse {
//                     id: d.id,
//                     device_id: d.device_id,
//                     name: d.name,
//                     device_type: d.device_type,
//                     apk_key: d.apk_key,
//                     business_id: d.business_id,
//                     account_id: d.account_id,
//                     created_at: d.created_at,
//                     updated_at: d.updated_at,
//                 })
//                 .collect();
//             Ok((StatusCode::OK, Json(response)).into_response())
//         }
//         Err(e) => Ok(e.into_response()),
//     }
// }

// /// Update a device
// pub async fn update_device(
//     State(store): State<(Store, Arc<Cache>)>,
//     Path(id): Path<Uuid>,
//     Json(packet): Json<DevicePostRequest>,
// ) -> Result<impl IntoResponse, Response> {
//     let updated_device = Device {
//         id,
//         main_id: Uuid::new_v4(),
//         device_id: packet.device_id,
//         name: packet.name,
//         device_type: packet.device_type,
//         apk_key: packet.apk_key,
//         business_id: packet.business_id,
//         account_id: packet.account_id,
//         created_at: Utc::now(), // You may want to fetch the old one instead
//         updated_at: Utc::now(),
//     };

//     match store.update_device(&updated_device).await {
//         Ok(_) => {
//             let response = DeviceResponse {
//                 id: updated_device.id,
//                 device_id: updated_device.device_id,
//                 name: updated_device.name,
//                 device_type: updated_device.device_type,
//                 apk_key: updated_device.apk_key,
//                 business_id: updated_device.business_id,
//                 account_id: updated_device.account_id,
//                 created_at: updated_device.created_at,
//                 updated_at: updated_device.updated_at,
//             };
//             Ok((StatusCode::OK, Json(response)).into_response())
//         }
//         Err(e) => Ok(e.into_response()),
//     }
// }

// /// Delete a device
// pub async fn delete_device(
//     State(store): State<(Store, Arc<Cache>)>,
//     Path(id): Path<Uuid>,
// ) -> Result<impl IntoResponse, Response> {
//     match store.delete_device(id).await {
//         Ok(_) => Ok(StatusCode::NO_CONTENT.into_response()),
//         Err(e) => Ok(e.into_response()),
//     }
// }
