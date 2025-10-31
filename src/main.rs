mod db_manager;
mod db_provider;
mod models;
mod router;

use crate::router::*;
use axum::{
    body::Body,
    http::Request,
    routing::{delete, get, post, put},
    Router,
};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin};
use tracing::{info, Span};

#[tokio::main]
async fn main() {
    env_logger::init();

    let api = Router::<()>::new()
        .route("/bands", get(get_bands))
        .route("/songs", get(get_songs))
        .route("/songs", post(post_songs))
        .route("/songs/{id}", get(get_song_id))
        .route("/songs/{id}", put(put_song_id))
        .route("/songs/{id}", delete(delete_song_id));

    let root = Router::new()
        .nest("/api", api)
        .layer(
            tower::ServiceBuilder::new().layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(AllowOrigin::any())
                    .allow_methods(AllowMethods::any())
                    .allow_headers(AllowHeaders::any()),
            ),
        )
        .layer(tower::ServiceBuilder::new().layer(
            tower_http::trace::TraceLayer::new_for_http().on_request(
                |request: &Request<Body>, _span: &Span| {
                    info!(
                        "{} {}",
                        request.method(),
                        request.uri().path_and_query().unwrap()
                    );
                },
            ),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, root).await.unwrap();
    println!("Hello, world!");
}
