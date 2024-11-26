use axum::{http::StatusCode, response::IntoResponse, Json};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{
    database::{self},
    types::Book,
};

/// [GET::/book]
pub async fn index() -> impl IntoResponse {
    let books = database::books::find_many(doc! {}).await;
    match books {
        Ok(books) => (StatusCode::OK, Json(books).into_response()),
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
    pub name: String,
    pub author: String,
    pub price: u32,
}

/// [POST::/book]
pub async fn post(Json(params): Json<PostQuery>) -> impl IntoResponse {
    let items = Book::new(params.name, params.author, params.price);

    match database::books::insert_one(&items).await {
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
            "author": items.author,
            "price": items.price
        }),
    )
        .into_response()
}
