use bytes::Bytes;
use chrono::{DateTime, Utc};
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountResponse {
    pub account_number: Vec<u8>,
    pub account_name: String,
    pub bank_id: String,
    pub bank_name: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: Uuid,
    pub bank_id: String,        // varchar → String
    pub account_name: String,
    pub account_number: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountItem {
    pub id: Vec<u8>,
    pub account_number: Vec<u8>,
    pub account_name: String,
    pub bank_id: String,
    pub bank_name: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct AccountItemStore {
    pub accounts: Vec<AccountItem>,
}


impl Store {
    pub async fn add_account(&self, account: &Account) -> Result<bool, Error> {
        let query = r#"
            INSERT INTO Accounts (id, business_id, bank_id, account_name, account_number, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#;
        sqlx::query(query)
            .bind(account.id)
            .bind(&account.bank_id)
            .bind(&account.account_name)
            .bind(&account.account_number)
            .bind(account.created_at)
            .bind(account.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_account(&self, id: &Uuid) -> Result<Account, Error> {
        sqlx::query("SELECT * FROM \"Accounts\" WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Account {
                id: row.get("id"),
                bank_id: row.get("bank_id"),
                account_name: row.get("account_name"),
                account_number: row.get("account_number"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_accounts_user_id(&self) -> Result<Vec<Account>, Error> {
        sqlx::query("SELECT * FROM Accounts")
            .map(|row: PgRow| Account {
                id: row.get("id"),
                bank_id: row.get("bank_id"),
                account_name: row.get("account_name"),
                account_number: row.get("account_number"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_accounts(&self) -> Result<Vec<Account>, Error> {
        sqlx::query("SELECT * FROM Accounts")
            .map(|row: PgRow| Account {
                id: row.get("id"),
                bank_id: row.get("bank_id"),
                account_name: row.get("account_name"),
                account_number: row.get("account_number"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn update_account(&self, account: &Account) -> Result<bool, Error> {
        let query = r#"
            UPDATE Accounts
            SET business_id = $2, bank_id = $3, account_name = $4, account_number = $5, updated_at = $6
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(account.id)
            .bind(&account.bank_id)
            .bind(&account.account_name)
            .bind(&account.account_number)
            .bind(account.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_account(&self, id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM Accounts WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}