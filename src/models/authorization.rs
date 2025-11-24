//! Authorization models for the OpenAI Proxy Carousel
//!
//! This module contains data structures related to access key authorization.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

/// Custom error type for authorization failures
#[derive(Debug)]
pub enum AuthorizationError {
    /// Returned when no Authorization header is present in the request
    MissingAuthorizationHeader,
    /// Returned when the Authorization header doesn't use the Bearer scheme
    InvalidAuthorizationScheme,
    /// Returned when the provided access key is not in the configured access_keys list
    Unauthorized,
}

impl IntoResponse for AuthorizationError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthorizationError::MissingAuthorizationHeader => {
                (StatusCode::UNAUTHORIZED, "Authorization header is missing")
            }
            AuthorizationError::InvalidAuthorizationScheme => {
                (StatusCode::UNAUTHORIZED, "Invalid authorization scheme")
            }
            AuthorizationError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Access key is invalid or missing")
            }
        };

        let body = Json(serde_json::json!({
            "error": "Unauthorized",
            "message": error_message,
        }));

        (status, body).into_response()
    }
}

/// A struct to hold a successfully validated access key
#[derive(Debug, Clone)]
pub struct ValidatedAccessKey {
    /// The validated access key string
    pub key: String,
}