use axum::response::IntoResponse;
use axum::http::StatusCode;

pub enum ErrorResponses {
    InternalServerError,
    RateLimitExceeded
}

impl IntoResponse for ErrorResponses {
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrorResponses::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            ErrorResponses::RateLimitExceeded => (StatusCode::TOO_MANY_REQUESTS).into_response()
        }
    }
}