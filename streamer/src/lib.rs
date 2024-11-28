pub mod controllers {
    pub mod ray_stream_ctrl;
}
pub mod routes {
    pub mod ray_stream_rt;
}
pub mod lib {
    pub mod app_state;
}

use axum::{extract::State, Router};
use dotenv::dotenv;
use lib::app_state::AppState;
use routes::ray_stream_rt::get_raydium_stream_router;
use std::{env, sync::Arc};
use solana_client::rpc_client::RpcClient;

pub async fn run() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let raydium_url = env::var("RAYDIUM_URL").unwrap();

    let state = State(Arc::new(
        AppState {
            client: RpcClient::new(raydium_url)
        }
    ));

    _ = axum::serve(
        tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap(),
        Router::new()
            .merge(get_raydium_stream_router(state))
    ).await;
}