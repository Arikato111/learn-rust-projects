use axum::{
    http::StatusCode,
    routing::{get, post},
    Router, ServiceExt,
};

use crate::controllers::items_controller;

pub fn web_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/item", get(items_controller::index))
        .route("/item", post(items_controller::post))
}
