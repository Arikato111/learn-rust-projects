use super::connect::dbconnect;
// private
mod read;
mod write;

// public
pub use read::{find_many, find_one};
pub use write::{insert_many, insert_one};

pub async fn books<T: Send + Sync>() -> mongodb::Collection<T> {
    dbconnect().await.collection("books")
}
