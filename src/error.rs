// Copyright (c) 2023 Nostr Development Kit Devs
// Distributed under the MIT software license

use axum::extract::rejection::JsonRejection;
use axum::extract::FromRequest;
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
    // The request body contained invalid JSON
    JsonRejection(JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::JsonRejection(rejection) => (rejection.status(), rejection.body_text()),
        };

        (
            status,
            AppJson(json!({
                "success": false,
                "code": status.as_u16(),
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
