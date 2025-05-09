use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde_json::json;

/// Reporesents application error
#[derive(Debug)]
pub enum AppStatus {
    /// Operation successful
    Ok(String),

    /// The request doesn't have valid authentication credentials
    Unauthenticated(Option<String>),

    /// Requested entity was not found
    NotFound(Option<String>),

    /// Operation is not implemented
    Unimplemented(),

    /// Internal error
    Internal(Option<String>),

    /// Entity already exists
    AlreadyExists(Option<String>),

    /// The client specified an invalid/malformed argument
    InvalidArgument(Option<String>),
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

            AppStatus::NotFound(msg) => {
                if let Some(msg) = msg {
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!({"code": 404, "msg": msg})),
                    )
                        .into_response()
                } else {
                    (
                        StatusCode::NOT_FOUND,
                        Json(json!({"code": 404, "msg": "Requested entity was not found!"})),
                    )
                        .into_response()
                }
            }

            AppStatus::Unimplemented() => (
                StatusCode::NOT_IMPLEMENTED,
                Json(json!({"code": 501, "msg": "Not Implemented!"})),
            )
                .into_response(),

            AppStatus::AlreadyExists(msg) => {
                if let Some(msg) = msg {
                    (StatusCode::CONFLICT, Json(json!({"code": 409, "msg": msg}))).into_response()
                } else {
                    (
                        StatusCode::CONFLICT,
                        Json(json!({"code": 409, "msg": "Entity alredy exists!"})),
                    )
                        .into_response()
                }
            }

            AppStatus::InvalidArgument(msg) => {
                if let Some(msg) = msg {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"code": 400, "msg": msg})),
                    )
                        .into_response()
                } else {
                    (
                        StatusCode::BAD_REQUEST,
                        Json(json!({"code": 400, "msg": "Invalid argument was specified!"})),
                    )
                        .into_response()
                }
            }
        }
    }
}
