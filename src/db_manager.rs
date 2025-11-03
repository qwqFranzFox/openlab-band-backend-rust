use crate::db_provider::DatabaseProvider;
use std::collections::HashMap;

type JsonObject = HashMap<String, String>;

pub struct DatabaseAccess {
    db_provider: Box<dyn DatabaseProvider>,
}

impl DatabaseAccess {
    pub fn new(db_provider: Box<dyn DatabaseProvider>) -> DatabaseAccess {
        return DatabaseAccess {
            db_provider: db_provider,
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
