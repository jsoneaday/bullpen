use std::{str::FromStr, sync::Arc};
use axum::{
    extract::{State, WebSocketUpgrade}, response::IntoResponse
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use solana_client::rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_transaction_status::UiTransactionEncoding;
use crate::lib::{app_state::AppState, ray_swaps::extract_swap_details};

pub async fn stream_ray_data(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        let (mut sender, mut receiver) = socket.split();

        loop {
            dotenv().ok();

            tokio::select! {
                Some(msg) = receiver.next() => {
                    match msg {
                        Ok(axum::extract::ws::Message::Close(_)) => {
                            println!("Client closed connection");
                            break;
                        },
                        Ok(axum::extract::ws::Message::Text(addr)) => {
                            let app_state = Arc::clone(&state);
                            let rpc_client = &app_state.rpc_client;
                            let ps_client = &app_state.ps_client;
                            let pool_key = Pubkey::from_str(&addr).unwrap();
                            let config = RpcTransactionLogsConfig {
                                commitment: Some(CommitmentConfig::confirmed())
                            };
                            let (mut tx_sub, _unsub) = ps_client
                                .logs_subscribe(RpcTransactionLogsFilter::Mentions(vec![addr]), config)
                                .await
                                .unwrap();

                            println!("Subscribed to pool: {}", pool_key);

                            while let Some(log_data) = tx_sub.next().await {
                                if log_data.value.logs.iter().any(|log| log.contains("Swap")) {
                                    if let Ok(tx_data) = rpc_client
                                        .get_transaction(&Signature::from_str(&log_data.value.signature).unwrap(), UiTransactionEncoding::Json) {
                                            println!("tx_data: {:?}", tx_data);
                                            let swap_info = extract_swap_details(&tx_data);
                                            if let Ok(json) = serde_json::to_string(&swap_info) {
                                                if let Err(e) = sender.send(axum::extract::ws::Message::Text(json)).await {
                                                    eprintln!("Failed to send tx swap data: {:?}", e);
                                                    break;
                                                }
                                            }
                                    }
                                }
                            }
                        },
                        Ok(_) => {
                            println!("An unknown request was made");
                            break;
                        },
                        Err(e) => {
                            println!("Error receiving message: {}", e);
                            break;
                        }
                    }
                }
            }
        }
    })
}