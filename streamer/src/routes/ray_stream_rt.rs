use std::sync::Arc;
use axum::{extract::State, routing::get, Router};
use crate::{controllers::ray_stream_ctrl::stream_ray_data, lib::app_state::AppState};

pub fn get_raydium_stream_router(State(state): State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/ray", get(stream_ray_data))
        .with_state(state)
}