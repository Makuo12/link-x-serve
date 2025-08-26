use chrono::{DateTime, Utc};
use handle_error::Error;
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

use crate::db_store::Store;

// ========== Structs ==========
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payment {
    pub device_id: Uuid,
    pub amount: i64,
    pub customer_id: Uuid,
    pub user_id: Uuid,
    pub bank_id: String,
    pub account_name: String,
    pub account_number: String,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PaymentResponse {
   device_id: String,
   amount: i64,
   bank_id: String,
   account_name: String,
   account_number: String,
   device_name: String,
   device_type: String,
   customer_first_name: String,
   customer_last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaymentSend {
    pub device_id: Uuid,
    pub amount: i64,
    pub customer_id: Uuid,
    pub user_id: Uuid,
    pub bank_id: String,
    pub account_name: String,
    pub account_number: String,
}

// ========== Store Implementation ==========
impl Store {
    pub async fn add_payment(
        &self,
        device_id: Uuid,
        amount: i64,
        customer_id: Uuid,
        user_id: Uuid,
        bank_id: String,
        account_name: String,
        account_number: String,
    ) -> Result<PaymentSend, Error> {
        let query = r#"
            INSERT INTO payments (device_id, amount, customer_id, user_id, bank_id, account_name, account_number)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING device_id, amount, customer_id, user_id, bank_id, account_name, account_number
        "#;

        let (device_id, amount, customer_id, user_id, bank_id, account_name, account_number): 
            (Uuid, i64, Uuid, Uuid, String, String, String) = sqlx::query_as(query)
            .bind(device_id)
            .bind(amount)
            .bind(customer_id)
            .bind(user_id)
            .bind(bank_id.clone())
            .bind(account_name.clone())
            .bind(account_number.clone())
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))?;

        Ok(PaymentSend {
            device_id,
            amount,
            customer_id,
            user_id,
            bank_id,
            account_name,
            account_number,
        })
    }

    pub async fn get_payments_user_id(&self, user_id: Uuid) -> Result<Vec<PaymentResponse>, Error> {
        let query = r#"
            SELECT
                p.device_id,
                p.amount,
                p.bank_id,
                p.account_name,
                p.account_number,
                d.name as device_name,
                d.device_type,
                c.first_name as customer_first_name,
                c.last_name as customer_last_name
            FROM payments p
            JOIN customers c ON c.id = p.customer_id
            JOIN devices_accessible d ON d.device_id = p.device_id
            WHERE p.user_id = $1
        "#;
        
        sqlx::query(query)
            .bind(user_id)
            .map(|row: PgRow| PaymentResponse {
                device_id: row.get("device_id"),
                amount: row.get("amount"),
                bank_id: row.get("bank_id"),
                account_name: row.get("account_name"),
                account_number: row.get("account_number"),
                device_name: row.get("device_name"),
                device_type: row.get("device_type"),
                customer_first_name: row.get("customer_first_name"),
                customer_last_name: row.get("customer_last_name"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    // pub async fn update_payment(
    //     &self,
    //     device_id: &Uuid,
    //     amount: &i64,
    //     customer_id: &Uuid,
    //     user_id: &Uuid,
    //     bank_id: &String,
    //     account_name: &String,
    //     account_number: &String,
    // ) -> Result<bool, Error> {
    //     let query = r#"
    //         UPDATE payments
    //         SET amount = $2, customer_id = $3, user_id = $4, bank_id = $5, account_name = $6, account_number = $7
    //         WHERE device_id = $1
    //     "#;

    //     sqlx::query(query)
    //         .bind(device_id)
    //         .bind(amount)
    //         .bind(customer_id)
    //         .bind(user_id)
    //         .bind(bank_id)
    //         .bind(account_name)
    //         .bind(account_number)
    //         .execute(&self.connection)
    //         .await
    //         .map(|_| true)
    //         .map_err(|e| Error::DatabaseQueryError(e))
    // }

    // pub async fn delete_payment(&self, device_id: Uuid) -> Result<bool, Error> {
    //     sqlx::query("DELETE FROM payments WHERE device_id = $1")
    //         .bind(device_id)
    //         .execute(&self.connection)
    //         .await
    //         .map(|_| true)
    //         .map_err(|e| Error::DatabaseQueryError(e))
    // }
}
