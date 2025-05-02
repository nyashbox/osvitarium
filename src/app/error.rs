use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use serde_json::json;

/// Reporesents application error
#[derive(Debug)]
pub enum AppStatus {
    /// Operation successful
    Ok(String),

    /// The request doesn't have valid authentication credentials
    Unauthenticated(Option<String>),

    /// Operation is not implemented
    Unimplemented(),

    /// Internal error
    Internal(Option<String>),
}

impl IntoResponse for AppStatus {
    fn into_response(self) -> Response {
        match self {
            AppStatus::Unauthenticated(msg) => {
                if let Some(m) = msg {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"code":401, "msg": m})),
                    )
                        .into_response()
                } else {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({"code": 401, "msg": "Invalid authentication credentials!"})),
                    )
                        .into_response()
                }
            }
            AppStatus::Internal(msg) => {
                if let Some(m) = msg {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"code":500, "msg": m})),
                    )
                        .into_response()
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"code":500, "msg": "Internal server error!"})),
                    )
                        .into_response()
                }
            }
            AppStatus::Ok(msg) => {
                (StatusCode::OK, Json(json!({"code": 200, "msg": msg}))).into_response()
            }

            AppStatus::Unimplemented() => (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({"code": 501, "msg": "Not Implemented!"})),
            )
                .into_response(),
        }
    }
}
