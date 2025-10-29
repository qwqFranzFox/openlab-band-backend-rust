mod db_manager;
mod models;

use std::{collections::HashMap, sync::LazyLock};

use db_manager::DatabaseAccess;
use models::QueryParam;

use axum::{
    extract::{Path, Query},
    http::{Response, StatusCode},
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};

use crate::db_manager::SqliteDatabase;

type APIResp = Response<axum::body::Body>;

use serde_json::json;

static DB_MANAGER: LazyLock<DatabaseAccess<SqliteDatabase>> =
    LazyLock::new(|| DatabaseAccess::new(SqliteDatabase::new(None)));

async fn get_bands(Query(query): QueryParam) -> APIResp {
    if let Some(name) = query.get("name") {
        return (
            StatusCode::OK,
            Json(json!(vec![DB_MANAGER
                .get_band_by_name(name.to_string())
                .await
                .unwrap()])),
        )
            .into_response();
    } else {
        return (
            StatusCode::OK,
            Json(json!(DB_MANAGER.get_all_bands().await.unwrap())),
        )
            .into_response();
    }
}

async fn get_songs(Query(query): QueryParam) -> APIResp {
    fn parse_get_songs_params(
        Query(query): QueryParam,
    ) -> (usize, usize, Option<String>, Option<String>) {
        let page_size: usize = query
            .get("page_size")
            .unwrap_or(&"10".to_string())
            .parse()
            .unwrap();
        let page_index: usize = query
            .get("page_index")
            .unwrap_or(&"1".to_string())
            .parse()
            .unwrap();
        let title = query.get("title");
        let band = query.get("band");
        (page_size, page_index, title.cloned(), band.cloned())
    }

    let (page_size, page_index, title, band) = parse_get_songs_params(Query(query));
    let total = {
        let b: Result<Vec<HashMap<String, String>>, _>;
        if let Some(title) = title {
            b = DB_MANAGER.get_song_by_title(title).await;
        } else if let Some(band) = band {
            b = DB_MANAGER.get_song_by_band(band).await;
        } else {
            b = DB_MANAGER.get_all_songs().await;
        }
        b
    };
    if let Ok(total) = total {
        let (current, status) = match total.chunks(page_size).nth(page_index - 1) {
            Some(val) => (Vec::from(val), StatusCode::OK),
            None => (Vec::new(), StatusCode::BAD_REQUEST),
        };
        return (
            status,
            Json(json!({
                "songs":current,
                "total":total.len(),
                "page_size":page_size,
                "page_index":page_index
            })),
        )
            .into_response();
    } else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json! {{}})).into_response();
    }
}

async fn get_song_by_id(Path(id): Path<usize>) -> APIResp {
    let k = DB_MANAGER.get_song_by_id(id).await;
    match k {
        Err(e) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json! {{}})).into_response();
        }
        Ok(data) => {
            if data.len() == 0 {
                return (StatusCode::NOT_FOUND, Json(json! {{"detail":"歌曲不存在"}}))
                    .into_response();
            } else {
                return (StatusCode::OK, Json(json! {data})).into_response();
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let api = Router::<()>::new()
        .route("/bands", get(get_bands))
        .route("/songs", get(get_songs))
        .route("/songs/{id}", get(get_song_by_id));

    let root = Router::new().nest("/api", api);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, root).await.unwrap();
    println!("Hello, world!");
}
