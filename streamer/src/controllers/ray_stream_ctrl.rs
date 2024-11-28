use std::{str::FromStr, sync::Arc};
use axum::{
    extract::{State, WebSocketUpgrade}, response::IntoResponse
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use solana_sdk::pubkey::Pubkey;
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use crate::lib::{app_state::AppState, ray_decoder::RaydiumPool};
use base64::prelude::*;

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
                            let ps_client = &app_state.ps_client;
                            // take pool address and use it to get pool's swap data
                            let pool_key = Pubkey::from_str(&addr).unwrap();
                            let (mut sub, _unsub) = ps_client
                                .account_subscribe(&pool_key, None)
                                .await
                                .unwrap();

                            println!("Subscribed to pool: {}", pool_key);

                            while let Some(data) = sub.next().await {
                                match data.value.data {
                                    UiAccountData::Binary(data, encoding) => {
                                        match encoding {
                                            UiAccountEncoding::Binary => {
                                                match RaydiumPool::decode(&BASE64_STANDARD.decode(&data).unwrap()) {
                                                    Ok(result_data) => {
                                                        match serde_json::to_string(&result_data) {
                                                            Ok(json_result) => {
                                                                if let Err(e) = sender.send(axum::extract::ws::Message::Text(json_result)).await {
                                                                    eprintln!("Error sending WebSocket message: {}", e);
                                                                    break;
                                                                }
                                                            },
                                                            Err(e) => {
                                                                eprintln!("Error serializing message: {}", e);
                                                                break;
                                                            }
                                                        }                                                        
                                                    },
                                                    Err(e) => {
                                                        eprintln!("Error decoding solana data: {}", e);
                                                        break;
                                                    }
                                                }
                                            },
                                            _ => eprintln!("Unexpected data encoding format")
                                        }
                                    },
                                    _ => eprintln!("Unexpected data format")
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