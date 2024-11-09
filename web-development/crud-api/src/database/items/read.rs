use crate::types::Item;
use mongodb::bson::Document;
use std::error::Error;
use tracing::info;

use super::items;

pub async fn find_one(fillter: Document) -> Option<Item> {
    let cols = items::<Item>().await;
    let result = cols.find_one(fillter).await;
    if let Ok(item) = result {
        return item;
    }
    info!("find one: failed");
    None
}

pub async fn find_many(fillter: Document) -> Result<Vec<Item>, Box<dyn Error>> {
    let cols = items::<Item>().await;
    let mut cursor = cols.find(fillter).await?;
    let mut items = Vec::new();

    while let Ok(r) = cursor.advance().await {
        if !r {
            break;
        }
        match cursor.deserialize_current() {
            Ok(it) => {
                items.push(it);
            }
            Err(e) => {
                info!("Error: {:?}", e);
                return Err(e.into());
            }
        }
    }

    Ok(items)
}
