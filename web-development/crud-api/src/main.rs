use crud_api::routes::web_routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    let _ = axum::serve(listener, web_routes()).await;
}
