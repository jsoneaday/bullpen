use axum::Router;
use dotenv::dotenv;
use std::env;

pub async fn run() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();

    _ = axum::serve(
        tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap(),
        Router::new()
    ).await;
}