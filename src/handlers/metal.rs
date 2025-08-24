use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Healthly {
    healthly: bool
}

pub async fn get_metal_health() -> Result<impl IntoResponse, Response> {
    Ok((StatusCode::OK, axum::Json(Healthly{healthly: true})).into_response())
}