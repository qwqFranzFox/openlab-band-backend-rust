use crate::models::BandRow;
use sqlx::{sqlite::SqliteConnection, Connection};
use std::collections::HashMap;

trait DatabaseProvider {
    async fn get_all_bands(&self) -> Result<Vec<HashMap<String, String>>, sqlx::Error>;
    async fn get_band_by_name(&self, name: String) -> Result<HashMap<String, String>, sqlx::Error>;
}

fn band_to_hash_map(data: &BandRow) -> HashMap<String, String> {
    let mut k: HashMap<String, String> = HashMap::new();
    k.insert("id".to_string(), data.0.to_string());
    k.insert("name".to_string(), data.1.to_string());
    k.insert("description".to_string(), data.2.to_string());
    k.insert("created_at".to_string(), data.3.to_string());
    k
}

pub struct SqliteDatabase {
    addr: String,
}

impl SqliteDatabase {
    pub fn new(addr: Option<String>) -> SqliteDatabase {
        match addr {
            Some(addr) => SqliteDatabase { addr },
            None => SqliteDatabase {
                addr: "../data/band.db".to_string(),
            },
        }
    }
}

impl DatabaseProvider for SqliteDatabase {
    async fn get_all_bands(&self) -> Result<Vec<HashMap<String, String>>, sqlx::Error> {
        let mut sql_conn = SqliteConnection::connect(self.addr.as_str()).await?;
        let k: Result<Vec<BandRow>, _> = sqlx::query_as("SELECT * FROM bands")
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(data.iter().map(|x| band_to_hash_map(x)).collect());
        } else {
            return Ok(vec![]);
        }
    }

    async fn get_band_by_name(&self, name: String) -> Result<HashMap<String, String>, sqlx::Error> {
        let mut sql_conn = SqliteConnection::connect(self.addr.as_str()).await?;
        let k: Result<BandRow, _> = sqlx::query_as("SELECT * FROM bands WHERE name = $1")
            .bind(name)
            .fetch_one(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(band_to_hash_map(&data));
        } else {
            return Ok(HashMap::new());
        }
    }
}

pub struct DatabaseAccess<T: DatabaseProvider> {
    db_provider: T,
}

impl<T> DatabaseAccess<T>
where
    T: DatabaseProvider,
{
    pub fn new(db_provider: T) -> DatabaseAccess<T> {
        return DatabaseAccess { db_provider };
    }

    pub async fn get_all_bands(&self) -> Result<Vec<HashMap<String, String>>, sqlx::Error> {
        self.db_provider.get_all_bands().await
    }
    pub async fn get_band_by_name(
        &self,
        name: String,
    ) -> Result<HashMap<String, String>, sqlx::Error> {
        self.db_provider.get_band_by_name(name).await
    }
}
