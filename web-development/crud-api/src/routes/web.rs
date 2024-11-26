use axum::{
    routing::{get, post},
    Router,
};

use crate::controllers::items_controller;

pub fn web_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/book", get(items_controller::index))
        .route("/book", post(items_controller::post))
}
