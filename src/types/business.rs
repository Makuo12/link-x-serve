use chrono::{DateTime, Utc};
use handle_error::Error;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::db_store::Store;

#[derive(Debug, Clone)]
pub struct Business {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub location: String,
    pub geolocation: (f64, f64), // Postgres POINT → tuple (x=lng, y=lat)
    pub lat: f64,
    pub long: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl Store {
    pub async fn add_business(&self, business: &Business) -> Result<bool, Error> {
        let query = r#"
            INSERT INTO businesses (id, user_id, name, location, geolocation, lat, lon, created_at, updated_at)
            VALUES ($1, $2, $3, $4, point($5, $6), $7, $8)
        "#;
        sqlx::query(query)
            .bind(business.id)
            .bind(business.user_id)
            .bind(&business.name)
            .bind(&business.location)
            .bind(business.geolocation.0)
            .bind(business.geolocation.1)
            .bind(business.lat)
            .bind(business.long)
            .bind(business.created_at)
            .bind(business.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
    pub async fn get_businesses_user_id(&self, user_id: Uuid) -> Result<Vec<Business>, Error> {
        sqlx::query("SELECT * FROM businesses WHERE user_id = $1")
            .bind(user_id)
            .map(|row: PgRow| {
                Business {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    location: row.get("location"),
                    geolocation: (0.0, 0.0),
                    lat: row.get("lat"),
                    long: row.get("lon"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .fetch_all(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn get_business(&self, id: Uuid) -> Result<Business, Error> {
        sqlx::query("SELECT * FROM businesses WHERE id = $1")
            .bind(id)
            .map(|row: PgRow| {
                Business {
                    id: row.get("id"),
                    user_id: row.get("user_id"),
                    name: row.get("name"),
                    location: row.get("location"),
                    geolocation: (0.0, 0.0),
                    lat: row.get("lat"),
                    long: row.get("lon"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                }
            })
            .fetch_one(&self.connection)
            .await
            .map_err(|e| Error::DatabaseQueryError(e))
}


    pub async fn update_business(&self, business: &Business) -> Result<bool, Error> {
        let query = r#"
            UPDATE businesses
            SET name = $2, location = $3, geolocation = point($4, $5), updated_at = $6
            WHERE id = $1
        "#;
        sqlx::query(query)
            .bind(business.id)
            .bind(&business.name)
            .bind(&business.location)
            .bind(business.geolocation.0)
            .bind(business.geolocation.1)
            .bind(business.updated_at)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }

    pub async fn delete_business(&self, id: Uuid) -> Result<bool, Error> {
        sqlx::query("DELETE FROM businesses WHERE id = $1")
            .bind(id)
            .execute(&self.connection)
            .await
            .map(|_| true)
            .map_err(|e| Error::DatabaseQueryError(e))
    }
}