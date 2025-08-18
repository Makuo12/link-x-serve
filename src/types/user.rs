use chrono::prelude::*;
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub exp: DateTime<Utc>,
    pub account_id: uuid::Uuid,
    pub nbf: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub hashed_password: String,
    pub first_name: String,
    pub email: String,
    pub last_name: String,
}

pub struct UserSend {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String
}

impl Store {
    pub async fn add_user(&self, first_name: String, hashed_password: String, last_name: String, email: String) -> Result<UserSend, Error> {
        let query = r#"
            INSERT INTO users (hashed_password, first_name, last_name, email)
            VALUES ($1, $2, $3, $4)
            RETURNING id
        "#;
        let (id,): (Uuid,) = sqlx::query_as(query)
            .bind(hashed_password)
            .bind(first_name.clone())
            .bind(email.clone())
            .bind(last_name.clone())
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))?;

        Ok(
            UserSend { id: id, first_name: first_name, last_name: last_name, email: email }
        )
    }

    pub async fn get_user(&self, id: Uuid) -> Result<User, Error> {
        sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| User {
                id: row.get("id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                hashed_password: row.get("hashed_password"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    
    pub async fn get_user_by_email(&self, email: String) -> Result<User, Error> {
        sqlx::query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .map(|row: PgRow| User {
                id: row.get("id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                hashed_password: row.get("hashed_password"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                email: row.get("email"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn update_user(&self, first_name: &String, last_name: &String, id: &Uuid) -> Result<bool, Error> {
        let query = r#"
            UPDATE users
            SET first_name = $2, last_name = $3, updated_at = $4
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(id)
            .bind(first_name)
            .bind(last_name)
            .bind(Utc::now())
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}