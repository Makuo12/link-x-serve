use chrono::{DateTime, Utc};
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;

#[derive(Debug, Clone)]
pub struct Device {
    pub main_id: Uuid,
    pub id: Uuid,
    pub device_id: String,
    pub name: String,
    pub account_id: Uuid,
    pub device_type: String,
    pub apk_key: String,
    pub price_key: String,
    pub id_key: String,
    pub business_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone)]
pub struct DeviceWithBusinessUserAccount {
    // Device
    pub main_device_id: Uuid,
    pub device_id: String,
    pub name: String,
    pub device_type: String,
    pub apk_key: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub price_key: String,
    pub id_key: String,
    // Business
    pub business_id: Uuid,
    pub business_name: String,
    pub business_location: String,

    // User
    pub user_id: Uuid,
    pub user_first_name: String,
    pub user_last_name: String,
    pub user_email: String,

    // Account
    pub account_id: Uuid,
    pub account_bank_id: String,
    pub account_name: String,
    pub account_number: String,
}




impl Store {
    pub async fn add_device(&self, device: &Device) -> Result<bool, Error> {
        let query = r#"
            INSERT INTO devices (id, device_id, name, type, business_id, created_at, updated_at, apk_key)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#;
        sqlx::query(query)
            .bind(device.id)
            .bind(&device.device_id)
            .bind(&device.name)
            .bind(&device.device_type)
            .bind(device.business_id)
            .bind(device.created_at)
            .bind(device.updated_at)
            .bind(&device.apk_key)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_device_with_business_user_account(
        &self,
        device_id: String,
    ) -> Result<DeviceWithBusinessUserAccount, Error> {
        let query = r#"
            SELECT 
                d.device_id,
                d.name,
                d.id AS main_device_id,
                d.device_type,
                d.apk_key,
                d.id_key,
                d.price_key,
                d.created_at,
                d.updated_at,
                b.id AS business_id,
                b.name AS business_name,
                b.location AS business_location,
                u.id AS user_id,
                u.first_name AS user_first_name,
                u.last_name AS user_last_name,
                u.email AS user_email,
                a.id AS account_id,
                a.bank_id AS account_bank_id,
                a.account_name AS account_name,
                a.account_number AS account_number
            FROM devices d
            JOIN businesses b ON d.business_id = b.id
            JOIN users u ON b.user_id = u.id
            JOIN "Accounts" a ON d.account_id = a.id
            WHERE d.device_id = $1
        "#;

        sqlx::query(query)
            .bind(device_id)
            .map(|row: PgRow| DeviceWithBusinessUserAccount {
                device_id: row.get("device_id"),
                name: row.get("name"),
                device_type: row.get("device_type"),
                apk_key: row.get("apk_key"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                business_id: row.get("business_id"),
                business_name: row.get("business_name"),
                business_location: row.get("business_location"),
                id_key: row.get("id_key"),
                price_key: row.get("price_key"),
                user_id: row.get("user_id"),
                main_device_id: row.get("main_device_id"),
                user_first_name: row.get("user_first_name"),
                user_last_name: row.get("user_last_name"),
                user_email: row.get("user_email"),
                account_id: row.get("account_id"),
                account_bank_id: row.get("account_bank_id"),
                account_name: row.get("account_name"),
                account_number: row.get("account_number"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_device(&self, id: Uuid) -> Result<Device, Error> {
        sqlx::query("SELECT * FROM devices WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Device {
                main_id: row.get("main_id"),
                id: row.get("id"),
                device_id: row.get("device_id"),
                name: row.get("name"),
                device_type: row.get("device_type"),
                business_id: row.get("business_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                apk_key: row.get("apk_key"),
                id_key: row.get("id_key"),
                price_key: row.get("price_key"),
                account_id: row.get("account_id"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn setup_device(&self, id: Uuid, device_id: String, price_key: String, id_key: String) -> Result<bool, Error> {
        let query = r#"
            UPDATE devices
            SET device_id = $2, price_key = $3, id_key = $4
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(id)
            .bind(device_id)
            .bind(price_key)
            .bind(id_key)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_devices(&self) -> Result<Vec<Device>, Error> {
        sqlx::query("SELECT * FROM devices")
            .map(|row: PgRow| Device {
                main_id: row.get("main_id"),
                id: row.get("id"),
                device_id: row.get("device_id"),
                name: row.get("name"),
                device_type: row.get("device_type"),
                business_id: row.get("business_id"),
                created_at: row.get("created_at"),
                id_key: row.get("id_key"),
                price_key: row.get("price_key"),
                updated_at: row.get("updated_at"),
                apk_key: row.get("apk_key"),
                account_id: row.get("account_id"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_devices_by_business_id(&self, business_id: &Uuid) -> Result<Vec<Device>, Error> {
        sqlx::query("SELECT * FROM devices WHERE business_id = $1")
            .bind(business_id)
            .map(|row: PgRow| Device {
                main_id: row.get("main_id"),
                id: row.get("id"),
                device_id: row.get("device_id"),
                id_key: row.get("id_key"),
                price_key: row.get("price_key"),
                name: row.get("name"),
                device_type: row.get("device_type"),
                business_id: row.get("business_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                apk_key: row.get("apk_key"),
                account_id: row.get("account_id"),
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn update_device(&self, device: &Device) -> Result<bool, Error> {
        let query = r#"
            UPDATE devices
            SET device_id = $2, name = $3, type = $4, business_id = $5, updated_at = $6, apk_key = $7
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(device.id)
            .bind(&device.device_id)
            .bind(&device.name)
            .bind(&device.device_type)
            .bind(device.business_id)
            .bind(device.updated_at)
            .bind(&device.apk_key)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_device(&self, main_id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM devices WHERE main_id = $1")
            .bind(main_id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}
