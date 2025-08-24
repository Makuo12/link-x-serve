use std::env;

use axum::http::response;
use handle_error::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Duration, Utc};

use crate::db_store::Store;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
}




impl Store {
    pub async fn refresh_access_token(&self, refresh_token: &str) -> Result<String, Error> {
        let session_key = env::var("SESSION_KEY")
            .map_err(|e| Error::EnvError(e))?;
        
        // Validate the refresh token
        let token_data = paseto::tokens::validate_local_token(
            &refresh_token,
            None,
            &session_key.as_bytes(),
            &paseto::tokens::TimeBackend::Chrono,
        )
        .map_err(|_| Error::CannotDecryptToken)?;
        
        // Extract and validate claims from refresh token
        let user_id: Uuid = token_data["id"]
            .as_str()
            .and_then(|s| Uuid::parse_str(s).ok())
            .ok_or(Error::InvalidSessionKey("Missing or invalid user id".to_string()))?;
        
        let token_type = token_data["token_type"]
            .as_str()
            .ok_or(Error::InvalidSessionKey("Missing token type".to_string()))?;
        
        // Ensure this is actually a refresh token
        if token_type != "refresh" {
            return Err(Error::InvalidSessionKey("Token is not a refresh token".to_string()));
        }
        
        // Generate new access token
        let now = Utc::now();
        let access_exp = now + Duration::minutes(15);
        
        let new_access_token = paseto::tokens::PasetoBuilder::new()
            .set_encryption_key(&Vec::from(session_key.as_bytes()))
            .set_expiration(&access_exp)
            .set_not_before(&now)
            .set_claim("id", serde_json::json!(user_id))
            .set_claim("token_type", serde_json::json!("access"))
            .build()
            .map_err(|e| Error::TokenCreationError(e.to_string()))?;
        let result = self.update_session(user_id, refresh_token.to_owned(), new_access_token.clone()).await;
        let updated = match result {
            Ok(u) => u,
            Err(e) => return Err(e),
        };
        if updated {
            Ok(new_access_token)
        } else {
            Ok("nothing".to_owned())
        }
    }
    pub async fn update_session(&self, user_id: Uuid, refresh_token: String, access_token: String) -> Result<bool, Error> {
        let query = r#"
            UPDATE sessions SET access_token = $3 
            WHERE id = $1 AND refresh_token = $2
        "#;
        sqlx::query(query)
        .bind(user_id)
        .bind(refresh_token)
        .bind(access_token)
        .execute(&self.connection)
        .await
        .map(|_| true)
        .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn create_session(&self, user_id: Uuid) -> Result<Session, Error> {
        let now = Utc::now();
        // Expiration aligned with refresh token (7 days)
        let expires_at = now + chrono::Duration::days(7);
        // Generate tokens
        let result = issue_token(user_id).map_err(|e| e);

        let tokens = match result {
            Ok(t) => t,
            Err(e) => return Err(e)
        };
        // Build session struct
        let session = Session {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            user_id,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        };

        // Insert into DB
        let query = r#"
            INSERT INTO sessions (id, created_at, updated_at, user_id, access_token, refresh_token, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#;

        sqlx::query(query)
            .bind(session.id)
            .bind(session.created_at)
            .bind(session.updated_at)
            .bind(session.user_id)
            .bind(&session.access_token)
            .bind(&session.refresh_token)
            .bind(session.expires_at)
            .execute(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))?;

        Ok(session)
    }
}



#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}

pub fn issue_token(id: Uuid) -> Result<TokenPair, Error> {
    let now = Utc::now();
    let session_key = env::var("SESSION_KEY") // Use string literal instead of const
        .map_err(|e| Error::EnvError(e))?;
    
    // Ensure key is proper length (32 bytes for local tokens)
    let key_bytes = if session_key.len() >= 32 {
        &session_key.as_bytes()[..32]
    } else {
        return Err(Error::InvalidSessionKey("Session key must be at least 32 bytes".to_string()));
    };
    
    // Access token (expires in 15 minutes)
    let access_exp = now + Duration::hours(24);
    let access_token = paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key_bytes))
        .set_expiration(&access_exp)
        .set_not_before(&now)
        .set_claim("id", serde_json::json!(id))
        .set_claim("token_type", serde_json::json!("access"))
        .build()
        .map_err(|e| Error::InvalidSessionKey(e.to_string()))?; // Handle error instead of expect
    
    // Refresh token (expires in 7 days)
    let refresh_exp = now + Duration::days(7);
    let refresh_token = paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key_bytes))
        .set_expiration(&refresh_exp)
        .set_not_before(&now)
        .set_claim("id", serde_json::json!(id))
        .set_claim("token_type", serde_json::json!("refresh"))
        .build()
        .map_err(|e| Error::InvalidSessionKey(e.to_string()))?; // Handle error instead of expect
    
    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}
