use crate::models::{BandRow, SongRow};
use sqlx::{sqlite::SqliteConnection, Connection};
use std::collections::HashMap;

type JsonObject = HashMap<String, String>;

trait DatabaseProvider {
    async fn get_all_bands(&self) -> Result<Vec<JsonObject>, sqlx::Error>;
    async fn get_band_by_name(&self, name: String) -> Result<JsonObject, sqlx::Error>;
    async fn get_all_songs(&self) -> Result<Vec<JsonObject>, sqlx::Error>;

    async fn get_song_by_title(&self, title: String) -> Result<Vec<JsonObject>, sqlx::Error>;

    async fn get_song_by_band(&self, band: String) -> Result<Vec<JsonObject>, sqlx::Error>;

    async fn get_song_by_id(&self, id: usize) -> Result<JsonObject, sqlx::Error>;
}

fn band_to_json_obj(data: &BandRow) -> JsonObject {
    let mut k: JsonObject = JsonObject::new();
    k.insert("id".to_string(), data.0.to_string());
    k.insert("name".to_string(), data.1.to_string());
    k.insert("description".to_string(), data.2.to_string());
    k.insert("created_at".to_string(), data.3.to_string());
    k
}

fn song_to_json_obj(data: &SongRow) -> JsonObject {
    let mut k: JsonObject = JsonObject::new();
    k.insert("id".to_string(), data.0.to_string());
    k.insert("title".to_string(), data.1.to_string());
    k.insert("author".to_string(), data.2.to_string());
    k.insert("lyrics".to_string(), data.3.to_string());
    k.insert("band".to_string(), data.4.to_string());
    k.insert("created_at".to_string(), data.5.to_string());
    k.insert("updated_at".to_string(), data.6.to_string());
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

impl SqliteDatabase {
    async fn get_connection(&self) -> Result<SqliteConnection, sqlx::Error> {
        SqliteConnection::connect(self.addr.as_str()).await
    }
}

impl DatabaseProvider for SqliteDatabase {
    async fn get_all_bands(&self) -> Result<Vec<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<BandRow>, _> = sqlx::query_as("SELECT * FROM bands")
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(data.iter().map(|x| band_to_json_obj(x)).collect());
        } else {
            return Ok(vec![]);
        }
    }

    async fn get_band_by_name(&self, name: String) -> Result<JsonObject, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<BandRow, _> = sqlx::query_as("SELECT * FROM bands WHERE name = $1")
            .bind(name)
            .fetch_one(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(band_to_json_obj(&data));
        } else {
            return Ok(HashMap::new());
        }
    }

    async fn get_all_songs(&self) -> Result<Vec<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs")
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(data.iter().map(|k| song_to_json_obj(k)).collect())
        } else {
            Ok(vec![])
        }
    }
    async fn get_song_by_title(&self, title: String) -> Result<Vec<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs WHERE title = $1")
            .bind(title)
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(data.iter().map(|k| song_to_json_obj(k)).collect())
        } else {
            Ok(vec![])
        }
    }

    async fn get_song_by_band(&self, band: String) -> Result<Vec<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs WHERE band = $1")
            .bind(band)
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(data.iter().map(|k| song_to_json_obj(k)).collect())
        } else {
            Ok(vec![])
        }
    }

    async fn get_song_by_id(&self, id: usize) -> Result<JsonObject, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<SongRow, _> = sqlx::query_as("SELECT * FROM songs WHERE id = $1")
            .bind(id.to_string())
            .fetch_one(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(song_to_json_obj(&data))
        } else {
            Ok(JsonObject::new())
        }
    }
}
enum DBProvider {
    Sqlite(String),
    File(String, String),
}

pub struct DatabaseAccess<T: DatabaseProvider> {
    db_provider: Box<T>,
}

impl<T> DatabaseAccess<T>
where
    T: DatabaseProvider,
{
    pub fn new(db_provider: T) -> DatabaseAccess<T> {
        return DatabaseAccess {
            db_provider: Box::new(db_provider),
        };
    }

    pub async fn get_all_bands(&self) -> Result<Vec<JsonObject>, sqlx::Error> {
        self.db_provider.get_all_bands().await
    }
    pub async fn get_band_by_name(&self, name: String) -> Result<JsonObject, sqlx::Error> {
        self.db_provider.get_band_by_name(name).await
    }

    pub async fn get_all_songs(&self) -> Result<Vec<JsonObject>, sqlx::Error> {
        self.db_provider.get_all_songs().await
    }

    pub async fn get_song_by_title(&self, title: String) -> Result<Vec<JsonObject>, sqlx::Error> {
        self.db_provider.get_song_by_title(title).await
    }

    pub async fn get_song_by_band(&self, band: String) -> Result<Vec<JsonObject>, sqlx::Error> {
        self.db_provider.get_song_by_band(band).await
    }

    pub async fn get_song_by_id(&self, id: usize) -> Result<JsonObject, sqlx::Error> {
        self.db_provider.get_song_by_id(id).await
    }
}
