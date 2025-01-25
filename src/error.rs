// Copyright (c) 2023 Yuki Kishimoto
// Copyright (c) 2023-2025 Rust Nostr Developers
// Distributed under the MIT software license

use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub enum AppError {
    // Too many filters were provided in the request
    FilterError(usize),
    // Too many filters were provided in the request
    EventIdNotFound,
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
    // An Nostr Client error occurred
    NostrClientError(nostr_sdk::client::Error),
    // An Nostr Event error occurred
    NostrEventError(nostr_sdk::event::Error),
    // A Redis error occurred
    RedisError(redis::RedisError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::FilterError(max_filters) => (
                StatusCode::BAD_REQUEST,
                format!("Too many filters (max allowed {max_filters})"),
            ),
            AppError::EventIdNotFound => {
                (StatusCode::BAD_REQUEST, String::from("Event ID not found"))
            }
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
            AppError::NostrClientError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::NostrEventError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
            AppError::RedisError(err) => (StatusCode::BAD_REQUEST, err.to_string()),
        };

        (
            status,
            AppJson(json!({
                "success": false,
                "message": message,
                "data": {}
            })),
        )
            .into_response()
    }
}

impl From<JsonRejection> for AppError {
    fn from(rejection: JsonRejection) -> Self {
        Self::JsonRejection(rejection)
    }
}

impl From<nostr_sdk::client::Error> for AppError {
    fn from(error: nostr_sdk::client::Error) -> Self {
        Self::NostrClientError(error)
    }
}

impl From<nostr_sdk::event::Error> for AppError {
    fn from(error: nostr_sdk::event::Error) -> Self {
        Self::NostrEventError(error)
    }
}

impl From<redis::RedisError> for AppError {
    fn from(error: redis::RedisError) -> Self {
        Self::RedisError(error)
    }
}
