use crate::models::{BandRow, SongRow};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqliteConnection, Connection};
use std::collections::HashMap;

type JsonObject = HashMap<String, String>;

pub trait DatabaseProvider {
    async fn get_all_bands(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error>;

    async fn get_band_by_name(&self, name: String) -> Result<Option<JsonObject>, sqlx::Error>;

    async fn get_all_songs(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error>;

    async fn get_song_by_title(
        &self,
        title: String,
    ) -> Result<Option<Vec<JsonObject>>, sqlx::Error>;

    async fn get_song_by_band(&self, band: String) -> Result<Option<Vec<JsonObject>>, sqlx::Error>;

    async fn get_song_by_id(&self, id: i64) -> Result<Option<JsonObject>, sqlx::Error>;

    async fn create_song(&self, data: serde_json::Value)
        -> Result<Option<JsonObject>, sqlx::Error>;

    async fn delete_song(&self, id: i64) -> Result<i64, sqlx::Error>;

    async fn update_song(
        &self,
        id: i64,
        data: serde_json::Value,
    ) -> Result<Option<JsonObject>, sqlx::Error>;
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
    async fn get_all_bands(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<BandRow>, _> = sqlx::query_as("SELECT * FROM bands")
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(Some(data.iter().map(|x| band_to_json_obj(x)).collect()));
        } else {
            return Ok(None);
        }
    }

    async fn get_band_by_name(&self, name: String) -> Result<Option<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<BandRow, _> = sqlx::query_as("SELECT * FROM bands WHERE name = $1")
            .bind(name)
            .fetch_one(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            return Ok(Some(band_to_json_obj(&data)));
        } else {
            return Ok(None);
        }
    }

    async fn get_all_songs(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs")
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(Some(data.iter().map(|k| song_to_json_obj(k)).collect()))
        } else {
            Ok(None)
        }
    }
    async fn get_song_by_title(
        &self,
        title: String,
    ) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs WHERE title = $1")
            .bind(title)
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(Some(data.iter().map(|k| song_to_json_obj(k)).collect()))
        } else {
            Ok(None)
        }
    }

    async fn get_song_by_band(&self, band: String) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<Vec<SongRow>, _> = sqlx::query_as("SELECT * FROM songs WHERE band = $1")
            .bind(band)
            .fetch_all(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(Some(data.iter().map(|k| song_to_json_obj(k)).collect()))
        } else {
            Ok(None)
        }
    }

    async fn get_song_by_id(&self, id: i64) -> Result<Option<JsonObject>, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        let k: Result<SongRow, _> = sqlx::query_as("SELECT * FROM songs WHERE id = $1")
            .bind(id.to_string())
            .fetch_one(&mut sql_conn)
            .await;
        if let Ok(data) = k {
            Ok(Some(song_to_json_obj(&data)))
        } else {
            Ok(None)
        }
    }

    async fn create_song(
        &self,
        data: serde_json::Value,
    ) -> Result<Option<JsonObject>, sqlx::Error> {
        #[derive(Serialize, Deserialize)]
        struct CreateSongModel {
            title: String,
            author: Option<String>,
            lyrics: Option<String>,
            band: String,
        }
        let data: CreateSongModel = serde_json::from_value(data).unwrap();
        let mut sql_conn = self.get_connection().await?;
        // check if band exists
        let band = self.get_band_by_name(data.band.clone()).await?;
        if let None = band {
            return Ok(None);
        }
        // title, author, lyrics, band
        let insert_result =
            sqlx::query("INSERT INTO songs (title,author,lyrics,band,created_at,updated_at) VALUES ($1,$2,$3,$4,$5,$6)")
                .bind(data.title)
                .bind(data.author)
                .bind(data.lyrics)
                .bind(data.band)
                .bind(chrono::Utc::now().to_string())
                .bind(chrono::Utc::now().to_string())
                .execute(&mut sql_conn)
                .await?;
        let id = insert_result.last_insert_rowid();

        return Ok(self.get_song_by_id(id).await.unwrap());
    }

    async fn delete_song(&self, id: i64) -> Result<i64, sqlx::Error> {
        let mut sql_conn = self.get_connection().await?;
        // has band?
        let _get_band: SongRow = sqlx::query_as("SELECT * from songs WHERE id = $1")
            .bind(id)
            .fetch_one(&mut sql_conn)
            .await?;
        sqlx::query("DELETE FROM songs WHERE id = $1")
            .bind(id)
            .execute(&mut sql_conn)
            .await?;
        return Ok(id);
    }

    async fn update_song(
        &self,
        id: i64,
        data: serde_json::Value,
    ) -> Result<Option<JsonObject>, sqlx::Error> {
        #[derive(Serialize, Deserialize)]
        struct UpdateSongModel {
            title: Option<String>,
            author: Option<String>,
            lyrics: Option<String>,
            band: Option<String>,
        }
        let data: UpdateSongModel = serde_json::from_value(data).unwrap();
        let mut sql_conn = self.get_connection().await?;
        let song = self.get_song_by_id(id).await?;
        if let None = song {
            return Ok(None);
        }
        if let Some(title) = data.title {
            sqlx::query("UPDATE songs SET title = $1 WHERE id = $2")
                .bind(title)
                .bind(id)
                .execute(&mut sql_conn)
                .await?;
        }
        if let Some(author) = data.author {
            sqlx::query("UPDATE songs SET author = $1 WHERE id = $2")
                .bind(author)
                .bind(id)
                .execute(&mut sql_conn)
                .await?;
        }
        if let Some(lyrics) = data.lyrics {
            sqlx::query("UPDATE songs SET lyrics = $1 WHERE id = $2")
                .bind(lyrics)
                .bind(id)
                .execute(&mut sql_conn)
                .await?;
        }
        if let Some(band) = data.band {
            sqlx::query("UPDATE songs SET band = $1 WHERE id = $2")
                .bind(band)
                .bind(id)
                .execute(&mut sql_conn)
                .await?;
        }
        sqlx::query("UPDATE songs SET updated_at = $1 WHERE id = $2")
            .bind(chrono::Utc::now().to_string())
            .bind(id)
            .execute(&mut sql_conn)
            .await?;
        return self.get_song_by_id(id).await;
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

    pub async fn get_all_bands(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        self.db_provider.get_all_bands().await
    }
    pub async fn get_band_by_name(&self, name: String) -> Result<Option<JsonObject>, sqlx::Error> {
        self.db_provider.get_band_by_name(name).await
    }

    pub async fn get_all_songs(&self) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        self.db_provider.get_all_songs().await
    }

    pub async fn get_song_by_title(
        &self,
        title: String,
    ) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        self.db_provider.get_song_by_title(title).await
    }

    pub async fn get_song_by_band(
        &self,
        band: String,
    ) -> Result<Option<Vec<JsonObject>>, sqlx::Error> {
        self.db_provider.get_song_by_band(band).await
    }

    pub async fn get_song_by_id(&self, id: i64) -> Result<Option<JsonObject>, sqlx::Error> {
        self.db_provider.get_song_by_id(id).await
    }

    pub async fn create_song(
        &self,
        data: serde_json::Value,
    ) -> Result<Option<JsonObject>, sqlx::Error> {
        self.db_provider.create_song(data).await
    }

    pub async fn delete_song(&self, id: i64) -> Result<i64, sqlx::Error> {
        self.db_provider.delete_song(id).await
    }

    pub async fn update_song(
        &self,
        id: i64,
        data: serde_json::Value,
    ) -> Result<Option<JsonObject>, sqlx::Error> {
        self.db_provider.update_song(id, data).await
    }
}
