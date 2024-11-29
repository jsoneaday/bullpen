pub mod controllers {
    pub mod ray_stream_ctrl;
}
pub mod routes {
    pub mod ray_stream_rt;
}
pub mod lib {
    pub mod app_state;
    pub mod ray_swaps;
    pub mod ray_pool;
    pub mod responses {
        pub mod app_response;
        pub mod error_response;
    }
}

use axum::{extract::State, Router};
use dotenv::dotenv;
use lib::app_state::AppState;
use routes::ray_stream_rt::get_raydium_stream_router;
use std::{env, sync::Arc};
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_client::RpcClient};

pub async fn run() {
    dotenv().ok();

    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let raydium_rpc_url = env::var("RAYDIUM_RPC_URL").unwrap();
    let raydium_wss_url = env::var("RAYDIUM_WSS_URL").unwrap();

    let state = State(Arc::new(
        AppState {
            rpc_client: RpcClient::new(raydium_rpc_url),
            ps_client: PubsubClient::new(&raydium_wss_url).await.unwrap()
        }
    ));

    _ = axum::serve(
        tokio::net::TcpListener::bind(format!("{}:{}", host, port)).await.unwrap(),
        Router::new()
            .merge(get_raydium_stream_router(state))
    ).await;
}