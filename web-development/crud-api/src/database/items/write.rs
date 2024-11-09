use super::items;
use crate::types::Item;
use mongodb::bson::{oid::ObjectId, Document};

pub async fn insert_one(item: &Item) -> Result<ObjectId, String> {
    let cols = items::<Document>().await;

    if let Ok(obj) = cols.insert_one(item.to_doc()).await {
        if let Some(v) = obj.inserted_id.as_object_id() {
            return Ok(v);
        }
    }
    Err("Failed to insert".to_string())
}

pub async fn insert_many(
    value: &Vec<Item>,
) -> Result<mongodb::results::InsertManyResult, mongodb::error::Error> {
    let cols = items::<Item>().await;

    cols.insert_many(value).await
}
