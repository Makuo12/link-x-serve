use chrono::{DateTime, Utc};
use handle_error::Error;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;
pub struct Bank {
    pub id: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub apk_key: String, // new field
}

impl Store {
    pub async fn add_bank(&self, bank: &Bank) -> Result<bool, Error> {
        let query = r#"
            INSERT INTO banks (id, user_id, created_at, updated_at, apk_key)
            VALUES ($1, $2, $3, $4, $5)
        "#;
        sqlx::query(query)
            .bind(&bank.id)
            .bind(bank.user_id)
            .bind(bank.created_at)
            .bind(bank.updated_at)
            .bind(&bank.apk_key)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_bank(&self, id: &str) -> Result<Bank, Error> {
        sqlx::query("SELECT * FROM banks WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Bank {
                id: row.get("id"),
                user_id: row.get("user_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                apk_key: row.get("apk_key"), // fetch new field
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    
    pub async fn get_banks(&self) -> Result<Vec<Bank>, Error> {
            sqlx::query("SELECT * FROM banks")
                .map(|row: PgRow| Bank {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    apk_key: row.get("apk_key"),
                })
                .fetch_all(&self.connection)
                .await
                .map_err(|e| Error::DatabaseQueryError(e))
        }
    pub async fn update_bank(&self, bank: &Bank) -> Result<bool, Error> {
        let query = r#"
            UPDATE banks
            SET user_id = $2, updated_at = $3, apk_key = $4
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(&bank.id)
            .bind(bank.user_id)
            .bind(bank.updated_at)
            .bind(&bank.apk_key)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_bank(&self, id: &str) -> Result<bool, Error> {
        sqlx::query("DELETE FROM banks WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}
