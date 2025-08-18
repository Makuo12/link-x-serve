use std::{env, sync::Arc};

use axum::{
    extract::{Request, State},
    http::{header::{self, AUTHORIZATION}, HeaderMap, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use handle_error::Error;
use tracing::info;
use uuid::Uuid;

use crate::{db_store::Store, tools::constant::{SESSION_KEY, XMINISTER_API_KEY, XMINISTER_METAL_API_KEY}, types::{api_key::ApiKey, cache::Cache}};

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, Error> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
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
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

// Extension trait to add user to request extensions
impl AuthenticatedUser {
    pub fn new(user_id: Uuid) -> Self {
        Self { user_id }
    }
}




pub async fn metal_apk(
State(cache): State<Arc<Cache>>, request: Request, next: Next
) -> Result<Response, Error> {
    handle_api_key(cache, ApiKeyType::XMinisterMetal,request, next).await
}

pub async fn public_apk(
State(cache): State<Arc<Cache>>, request: Request, next: Next
) -> Result<Response, Error> {
    handle_api_key(cache, ApiKeyType::XMinister,request, next).await
}

pub async fn handle_api_key(cache: Arc<Cache>, key_type: ApiKeyType, request: Request, next: Next) -> Result<Response, Error> {
    let key;
    let mut key_found = false;
    
    match key_type {
        ApiKeyType::XMinister => {
            key = env::var(XMINISTER_API_KEY)
                .map_err(|e| Error::EnvError(e))?;
            let list = cache.banks.try_read()
                .map_err(|e| Error::Unauthorized)?;
            for api_key in list.iter() {
                if api_key.apk_key == key {
                    key_found = true;
                    break;
                }
            }
        },
        ApiKeyType::XMinisterMetal => {
            key = env::var(XMINISTER_METAL_API_KEY)
                .map_err(|e| Error::EnvError(e))?;
            let list = cache.devices.try_read()
                .map_err(|e| Error::Unauthorized)?;
            for api_key in list.iter() {
                if api_key.apk_key == key {
                    key_found = true;
                    break;
                }
            }
        },
    }
    
    if key_found {
        Ok(next.run(request).await)
    } else {
        Err(Error::ApiKeyRejection) // Return error instead of calling next()
    }
}


pub enum ApiKeyType {
    XMinister,
    XMinisterMetal,
}