mod db_manager;
mod models;

use std::sync::Arc;

use db_manager::DatabaseAccess;
use models::QueryParam;

use axum::{extract::Query, response::Json, routing::get, Router};

use crate::db_manager::SqliteDatabase;

#[tokio::main]
async fn main() {
    let db = Arc::new(DatabaseAccess::new(SqliteDatabase::new(None)));
    let get_bands = |Query(query): QueryParam| async move {
        let db = &db;
        if let Some(name) = query.get("name") {
            let k = db
                .get_band_by_name(name.as_str().to_string())
                .await
                .unwrap();
            Json(vec![k])
        } else {
            let k = db.get_all_bands().await.unwrap();
            Json(k)
        }
    };
    let api = Router::<()>::new().route("/bands", get(get_bands));

    let root = Router::new().nest("/api", api);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, root).await.unwrap();
    println!("Hello, world!");
}
