use std::{env, sync::Arc};

use argon2::Config;
use axum::{extract::State, http::StatusCode, response::{IntoResponse, Response}, Extension, Json};
use chrono::{DateTime, Utc};
use handle_error::Error;
use rand::Rng;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{db_store::Store, handlers::middleware::AuthenticatedUser, tools::constant::SESSION_KEY, types::cache::Cache};


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserPostRequest {
    pub hashed_password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshTokenResponse {
    access_token: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RefreshTokenRequest {
    refresh_token: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub session: SessionResponse
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserOnlyResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}

fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify_password(
    hash: &str,
    password: &[u8],
) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

pub fn verify_token(
    token: String,
) -> Result<Uuid, Error> {
    let session_key = env::var(SESSION_KEY)
    .map_err(|e| Error::EnvError(e))?;
    let token_data = paseto::tokens::validate_local_token(
        &token,
        None,
        &session_key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| Error::CannotDecryptToken)?;

    let user_id: Uuid = token_data["id"]
        .as_str()
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or(Error::TokenCreationError("Could not generate token".to_string()))?;
        
    Ok(user_id)
}

pub async fn register(State(state): State<(Store, Arc<Cache>)>, Json(packet): Json<UserPostRequest>) ->Result<impl IntoResponse, Response> {
    let store = state.0;
    let hashed_password = hash_password(packet.hashed_password.as_bytes());
    let result = store.add_user(packet.first_name, hashed_password, packet.last_name, packet.email).await;
    match result {
        Ok(user_send) => {
            let session_result = store.create_session(user_send.id).await;
            let session = match session_result {
                Ok(s) => s,
                Err(e) => return Ok(e.into_response()),
            };
            let response = UserResponse {
                first_name: user_send.first_name,
                last_name: user_send.last_name,
                email: user_send.email,
                session: SessionResponse { access_token: session.access_token, refresh_token: session.refresh_token, expires_at: session.expires_at}
            };
            return Ok((StatusCode::CREATED, Json(response)).into_response());
        },
        Err(e) => {
            return Ok(e.into_response())
        }
    }
}
pub async fn update_user(State(state): State<(Store, Arc<Cache>)>, Extension(user): Extension<AuthenticatedUser>, Json(packet): Json<UserPostRequest>) ->Result<impl IntoResponse, Response> {
    let store = state.0;
    let result_user = store.get_user(user.user_id).await;
    
    let user_data = match result_user {
        Ok(u) => u,
        Err(e) => return Ok(e.into_response()),
    };

    let result = store.update_user(&packet.first_name, &packet.email, &packet.last_name, &user_data.id).await;
    match result {
        Ok(updated) => {
            if updated {
                let session_result = store.create_session(user_data.id).await;
                let session = match session_result {
                    Ok(s) => s,
                    Err(e) => return Ok(e.into_response()),
                };
                let response = UserResponse {
                    first_name: packet.first_name,
                    last_name: packet.last_name,
                    email: packet.email,
                    session: SessionResponse { access_token: session.access_token, refresh_token: session.refresh_token, expires_at: session.expires_at}
                };
                return Ok((StatusCode::CREATED, Json(response)).into_response());
            } else {
                return Ok(Error::Unauthorized.into_response())
            }
        },
        Err(e) => {
            return Ok(e.into_response())
        }
    }
}
pub async fn refresh_token(State(state): State<(Store, Arc<Cache>)>, Json(packet): Json<RefreshTokenRequest>) -> Result<impl IntoResponse, Response> {
    let store = state.0;
    let result = store.refresh_access_token(&packet.refresh_token).await;
    match result {
        Ok(token) => Ok((StatusCode::OK, Json(RefreshTokenResponse{access_token: token})).into_response()),
        Err(e) => Ok(e.into_response())
    }
}

pub async fn login(State(state): State<(Store, Arc<Cache>)>, Json(packet): Json<UserPostRequest>) ->Result<impl IntoResponse, Response> {
    let store = state.0;
    let result_user = store.get_user_by_email(packet.email).await; 
    let user = match result_user {
        Ok(u) => u,
        Err(e) => return Ok(e.into_response()),
    };
    let result_verified = verify_password(&user.hashed_password, &packet.hashed_password.as_bytes());
    match result_verified {
        Ok(verified) => {
            if verified {
                let session_result = store.create_session(user.id).await;
                let session = match session_result {
                    Ok(s) => s,
                    Err(e) => return Ok(e.into_response()),
                };
                let response = UserResponse {
                    first_name: user.first_name,
                    last_name: user.last_name,
                    email: user.email,
                    session: SessionResponse { access_token: session.access_token, refresh_token: session.refresh_token, expires_at: session.expires_at}
                };
                return Ok((StatusCode::OK, Json(response)).into_response());
            } else {
                return Ok(Error::WrongPassword.into_response())
            }
        },
        Err(e) => {
            return Ok(Error::ArgonLibraryError(e).into_response())
        }
    }
}


// Get current user (requires authentication)
pub async fn get_user_profile(
    State(state): State<(Store, Arc<Cache>)>, 
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<impl IntoResponse, Response> {
    // Get user by ID from the authenticated token
    let store = state.0;
    let result_user = store.get_user(user.user_id).await;
    
    let user_data = match result_user {
        Ok(u) => u,
        Err(e) => return Ok(e.into_response()),
    };
    
    let response = UserOnlyResponse {
        first_name: user_data.first_name,
        last_name: user_data.last_name,
        email: user_data.email,
    };
    return Ok((StatusCode::OK, Json(response)).into_response());
}
