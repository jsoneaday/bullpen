use std::{str::FromStr, sync::Arc};
use axum::{
    extract::{State, WebSocketUpgrade}, response::IntoResponse
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcTransactionLogsConfig, RpcTransactionLogsFilter};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey, signature::Signature};
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use solana_transaction_status::UiTransactionEncoding;
use crate::lib::{app_state::AppState, ray_pool::RaydiumPool, ray_swaps::extract_swap_details};
use base64::prelude::*;

pub async fn stream_ray_data(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        let (mut sender, mut receiver) = socket.split();
        let app_state = Arc::clone(&state);
        let rpc_client = &app_state.rpc_client;
        let ps_client = &app_state.ps_client;

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
                            let pool_key = Pubkey::from_str(&addr).unwrap();
                            
                            let tx_config = RpcTransactionLogsConfig {
                                commitment: Some(CommitmentConfig::confirmed())
                            };
                            let (mut tx_sub, _tx_unsub) = ps_client
                                .logs_subscribe(RpcTransactionLogsFilter::Mentions(vec![addr.clone()]), tx_config)
                                .await
                                .unwrap();
                            
                            while let Some(log_data) = tx_sub.next().await {
                                println!("next log");
                                if log_data.value.logs.iter().any(|log| log.contains("Swap")) {
                                    println!("log with Swap found!");
                                    if let Ok(tx_data) = rpc_client.get_transaction(
                                        &Signature::from_str(&log_data.value.signature).unwrap(), 
                                        UiTransactionEncoding::Json
                                    ) {    
                                        println!("tx_data found");
                                        if let Ok(account) = rpc_client.get_account(&pool_key) {
                                            let ray_pool = RaydiumPool::decode(&account.data).unwrap();
                                            println!("ray_pool: {:?}", ray_pool);
                                            let swap_info = extract_swap_details(&tx_data, &ray_pool);
                                            println!("swap_info: {:?}", swap_info);
                                            if let Ok(json) = serde_json::to_string(&swap_info) {
                                                if let Err(e) = sender.send(axum::extract::ws::Message::Text(json)).await {
                                                    eprintln!("Failed to send tx swap data: {:?}", e);
                                                    break;
                                                }
                                            }
                                        } else {
                                            println!("account not found");
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