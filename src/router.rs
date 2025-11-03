use std::sync::LazyLock;

use crate::db_manager::DatabaseAccess;
use crate::db_provider::SqliteDatabase;
use crate::models::QueryParam;
use axum::{
    extract::{Path, Query},
    http::{Response, StatusCode},
    response::{IntoResponse, Json},
};
use serde_json::json;
use tracing::error;

type APIResp = Response<axum::body::Body>;

static DB_MANAGER: LazyLock<DatabaseAccess> = LazyLock::new(|| {
    return DatabaseAccess::new(Box::new(SqliteDatabase::new(None)));
});
pub async fn get_bands(Query(query): QueryParam) -> APIResp {
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

pub async fn get_songs(Query(query): QueryParam) -> APIResp {
    let (page_size, page_index, title, band) = {
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
    };
    let total = {
        let b;
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
        if let Some(total) = total {
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
            return (StatusCode::NOT_FOUND, Json(json! {{"detail":"歌曲不存在"}})).into_response();
        }
    } else {
        let e = total.err().unwrap();
        error!("{}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json! {{}})).into_response();
    }
}

pub async fn get_song_id(Path(id): Path<i64>) -> APIResp {
    let k = DB_MANAGER.get_song_by_id(id).await;
    match k {
        Err(e) => {
            error!("{}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json! {{}})).into_response();
        }
        Ok(data) => {
            if let Some(data) = data {
                return (StatusCode::OK, Json(json! {data})).into_response();
            } else {
                return (StatusCode::NOT_FOUND, Json(json! {{"detail":"歌曲不存在"}}))
                    .into_response();
            }
        }
    }
}

pub async fn post_songs(Json(json_data): Json<serde_json::Value>) -> APIResp {
    let k = DB_MANAGER.create_song(json_data).await;
    return match k {
        Ok(value) => {
            if let Some(val) = value {
                return (StatusCode::OK, Json(json! {val})).into_response();
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(json! {{"detail":"请求格式错误"}}),
                )
                    .into_response();
            }
        }
        Err(e) => {
            error!("{}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"detail":"INTERNAL_SERVER_ERROR"})),
            )
                .into_response()
        }
    };
}

pub async fn put_song_id(Path(id): Path<i64>, Json(json_data): Json<serde_json::Value>) -> APIResp {
    let k = DB_MANAGER.update_song(id, json_data).await;
    match k {
        Ok(song) => match song {
            Some(song_data) => {
                return (StatusCode::OK, Json(json!(song_data))).into_response();
            }
            None => {
                return (StatusCode::NOT_FOUND, Json(json!({"detail":"歌曲不存在"})))
                    .into_response()
            }
        },
        Err(e) => {
            error!("{}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({}))).into_response();
        }
    };
}

pub async fn delete_song_id(Path(id): Path<i64>) -> APIResp {
    let k = DB_MANAGER.delete_song(id).await;
    match k {
        Ok(_id) => return (StatusCode::NO_CONTENT, Json(json!({}))).into_response(),
        Err(e) => {
            error!("{}", e);
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"detail":"歌曲不存在"})),
            )
                .into_response();
        }
    };
}
