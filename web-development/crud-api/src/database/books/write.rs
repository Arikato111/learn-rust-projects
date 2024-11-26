use super::books;
use crate::types::Book;
use mongodb::bson::{oid::ObjectId, Document};

pub async fn insert_one(book: &Book) -> Result<ObjectId, String> {
    let cols = books::<Document>().await;

    if let Ok(obj) = cols.insert_one(book.to_doc()).await {
        if let Some(v) = obj.inserted_id.as_object_id() {
            return Ok(v);
        }
    }
    Err("Failed to insert".to_string())
}

pub async fn insert_many(
    value: &Vec<Book>,
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {
    let cols = books::<Book>().await;

    cols.insert_many(value).await
}
