use std::{str::FromStr, sync::Arc};
use axum::{
    extract::{State, WebSocketUpgrade}, response::Response
};
use dotenv::dotenv;
use futures::{sink::SinkExt, stream::StreamExt};
use solana_sdk::pubkey::Pubkey;
use crate::lib::app_state::AppState;


pub async fn stream_ray_data(State(state): State<Arc<AppState>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| async move {
        let (mut sender, mut receiver) = socket.split();

        loop {
            dotenv().ok();

            tokio::select! {
                _ = sender.send(axum::extract::ws::Message::Text("hello world".to_string())) => {},
                Some(msg) = receiver.next() => {
                    match msg {
                        Ok(axum::extract::ws::Message::Close(_)) => {
                            println!("Client closed connection");
                            break;
                        },
                        Ok(axum::extract::ws::Message::Text(msg)) => {
                            let app_state = Arc::clone(&state);
                            let client = &app_state.client;
                            // take pool address and use it to get pool's swap data
                            let pool_key = Pubkey::from_str(&msg).unwrap();
                            match client.get_account_data(&pool_key) {
                                Ok(data) => {

                                },
                                Err(e) => println!("Error fetching pool data: {}", e)
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