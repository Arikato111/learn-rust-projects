use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Item {
    pub _id: ObjectId,
    pub name: String,
    pub age: i32,
}

impl Item {
    pub fn id_doc(&self) -> Document {
        doc! {
            "_id": self._id
        }
    }
    pub fn to_doc(&self) -> Document {
        doc! {
            "name": self.name.clone(),
            "age": self.age
        }
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    /// Creates a new [`Item`].
    pub fn new(name: String, age: i32) -> Self {
        Self {
            _id: ObjectId::new(),
            name,
            age,
        }
    }
}

impl Default for Item {
    fn default() -> Self {
        Self {
            _id: ObjectId::new(),
            name: String::default(),
            age: i32::default(),
        }
    }
}
