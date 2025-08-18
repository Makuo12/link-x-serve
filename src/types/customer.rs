use chrono::prelude::*;
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub public_key: String,
    pub private_key: String,
    pub bank_id: String, // varchar references banks.id
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Store {
    pub async fn add_customer(&self, customer: &Customer) -> Result<bool, Error> {
        let query = r#"
            INSERT INTO customers (id, first_name, last_name, public_key, private_key, bank_id, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;
        sqlx::query(query)
            .bind(customer.id)
            .bind(&customer.first_name)
            .bind(&customer.last_name)
            .bind(&customer.public_key)
            .bind(&customer.private_key)
            .bind(&customer.bank_id)
            .bind(customer.created_at)
            .bind(customer.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_customer(&self, id: Uuid) -> Result<Customer, Error> {
        sqlx::query("SELECT * FROM customers WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Customer {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                public_key: row.get("public_key"),
                private_key: row.get("private_key"),
                bank_id: row.get("bank_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_customers(&self) -> Result<Vec<Customer>, Error> {
        sqlx::query("SELECT * FROM customers")
            .map(|row: PgRow| Customer {
                id: row.get("id"),
                first_name: row.get("first_name"),
                last_name: row.get("last_name"),
                public_key: row.get("public_key"),
                private_key: row.get("private_key"),
                bank_id: row.get("bank_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn update_customer(&self, customer: &Customer) -> Result<bool, Error> {
        let query = r#"
            UPDATE customers
            SET first_name = $2, last_name = $3, public_key = $4, private_key = $5, bank_id = $6, updated_at = $7
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(customer.id)
            .bind(&customer.first_name)
            .bind(&customer.last_name)
            .bind(&customer.public_key)
            .bind(&customer.private_key)
            .bind(&customer.bank_id)
            .bind(customer.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_customer(&self, id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM customers WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}