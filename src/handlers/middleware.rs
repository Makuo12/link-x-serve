use std::{env, sync::Arc};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use handle_error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db_store::Store, tools::constant::{SESSION_KEY, XMINISTER_API_KEY, XMINISTER_METAL_API_KEY}, types::cache::Cache};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, Error> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)
        .map_err(|_| Error::CannotDecryptToken)?;
    let token = extract_token_from_headers(auth_header)?;
    
    // Validate the access token and extract user ID
    let user_id = extract_user_from_access_token(&token)?;
    
    // Add authenticated user to request extensions
    request.extensions_mut().insert(AuthenticatedUser::new(user_id));
    
    // Continue to the next middleware/handler
    Ok(next.run(request).await)
}

fn extract_user_from_access_token(token: &str) -> Result<Uuid, Error> {
    let session_key = env::var(SESSION_KEY)
        .map_err(|e| Error::EnvError(e))?;
    
    // Validate the access token
    let token_data = paseto::tokens::validate_local_token(
        token,
        None,
        &session_key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| Error::CannotDecryptToken)?;
    
    // Extract user ID from token data
    let user_id: Uuid = token_data["id"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or(Error::InvalidSessionKey("Missing or invalid user id".to_string()))?;
    
    Ok(user_id)
}

fn extract_token_from_headers(auth_header: &str) -> Result<String, Error> {
    if !auth_header.starts_with("Bearer ") {
        return Err(Error::CannotDecryptToken);
    }
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(Error::CannotDecryptToken)?
        .to_string();
    
    Ok(token)
}

// User context that will be available in your handlers
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthenticatedApk {
    pub apk: String,
}
// Extension trait to add user to request extensions
impl AuthenticatedUser {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}

impl AuthenticatedApk {
    pub fn new(apk: String) -> Self {
        Self { apk }
    }
}

pub async fn metal_apk(mut request: Request, next: Next
) -> Result<Response, Error> {
    let auth_header = request
        .headers()
        .get(XMINISTER_METAL_API_KEY)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)
        .map_err(|_| Error::CannotDecryptToken)?;
    let key = auth_header.to_string();
    request.extensions_mut().insert(AuthenticatedApk::new(key));
    Ok(next.run(request).await)
}

pub async fn public_apk(mut request: Request, next: Next
) -> Result<Response, Error> {
    let auth_header = request
        .headers()
        .get(XMINISTER_API_KEY)
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)
        .map_err(|_| Error::CannotDecryptToken)?;
    let key = auth_header.to_string();
    request.extensions_mut().insert(AuthenticatedApk::new(key));
    Ok(next.run(request).await)
}



pub enum ApiKeyType {
    XMinister,
    XMinisterMetal,
}