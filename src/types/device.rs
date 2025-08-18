use chrono::{DateTime, Utc};
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;

#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub device_id: String,
    pub name: String,
    pub account_id: Uuid,
    pub device_type: String,
    pub apk_key: String,
    pub business_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            .bind(&device.account_id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_device(&self, id: Uuid) -> Result<Device, Error> {
        sqlx::query("SELECT * FROM devices WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| Device {
                id: row.get("id"),
                device_id: row.get("device_id"),
                name: row.get("name"),
                device_type: row.get("type"),
                business_id: row.get("business_id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                apk_key: row.get("apk_key"),
                account_id: row.get("account_id"),
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_devices(&self) -> Result<Vec<Device>, Error> {
        sqlx::query("SELECT * FROM devices")
            .map(|row: PgRow| Device {
                id: row.get("id"),
                device_id: row.get("device_id"),
                name: row.get("name"),
                device_type: row.get("type"),
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
            .bind(&device.account_id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_device(&self, id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM devices WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}
