use axum::{response::IntoResponse, Json, http::StatusCode};
use serde::Serialize;

pub enum AppResponses<T: Serialize> {
    JsonData(T)
}

impl<T: Serialize> IntoResponse for AppResponses<T> {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppResponses::JsonData(data) => (StatusCode::OK, Json(data)).into_response()
        }
    }
}