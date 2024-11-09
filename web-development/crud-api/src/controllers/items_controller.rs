use axum::{
    extract::{rejection::JsonDataError, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    database::{self, items::items},
    types::Item,
};

/// [GET::/item]
pub async fn index() -> impl IntoResponse {
    let items = database::items::find_many(doc! {}).await;
    match items {
        Ok(items) => (StatusCode::OK, Json(items).into_response()),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "message": "bad request"
            }))
            .into_response(),
        ),
    }
}

#[derive(Serialize, Deserialize)]
pub struct PostQuery {
    pub name: Option<String>,
    pub age: Option<i32>,
}

/// [POST::/item]
pub async fn post(Json(params): Json<PostQuery>) -> impl IntoResponse {
    let items = if let (Some(name), Some(age)) = (params.name, params.age) {
        Item::new(name, age)
    } else {
        return (StatusCode::BAD_REQUEST, "Bad request").into_response();
    };

    match database::items::insert_one(&items).await {
        Ok(id) => {
            println!("{}", id);
        }
        Err(msg) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response();
        }
    }

    (
        StatusCode::OK,
        Json(doc! {
            "name": items.name,
            "age": items.age
        }),
    )
        .into_response()
}
