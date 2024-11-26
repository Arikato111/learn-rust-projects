use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Book {
    pub _id: ObjectId,
    pub name: String,
    pub author: String,
    pub price: u32,
}

impl Book {
    pub fn id_doc(&self) -> Document {
        doc! {
            "_id": self._id
        }
    }
    pub fn to_doc(&self) -> Document {
        doc! {
            "name": self.name.clone(),
            "author": self.author.clone(),
            "price": self.price
        }
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    /// Creates a new [`Item`].
    pub fn new(name: String, author: String, price: u32) -> Self {
        Self {
            _id: ObjectId::new(),
            name,
            author,
            price,
        }
    }
}

impl Default for Book {
    fn default() -> Self {
        Self {
            _id: ObjectId::new(),
            name: String::default(),
            author: String::default(),
            price: u32::default(),
        }
    }
}
