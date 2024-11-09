use mongodb::{Client, Database};

const URI: &str = "mongodb://root:admin123@127.0.0.1:27017/";

pub async fn dbconnect() -> Database {
    let client = match Client::with_uri_str(URI).await {
        Ok(cli) => cli,
        Err(e) => {
            tracing::info!("Error cannot connect Database: {:?}", e);
            panic!("Error cannot connect Database: {:?}", e);
        }
    };
    tracing::info!("connect Database success");
    client.database("crud")
}
