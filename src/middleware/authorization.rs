//! Authorization middleware for the OpenAI Proxy Carousel
//!
//! This module contains middleware for validating access keys in the Authorization header
//! when the server is configured to listen on "0.0.0.0".

use axum::{
    body::Body, http::Request, middleware::Next, response::{IntoResponse, Response}
};
use axum::http::header::{AUTHORIZATION, HeaderValue};

use crate::models::{AuthorizationError, ValidatedAccessKey};
use crate::state::State as ProxyState;

/// Extracts the access key from the Authorization header
///
/// # Arguments
///
/// * `header_value` - The Authorization header value
///
/// # Returns
///
/// * `Some(String)` - The access key if the header uses the Bearer scheme
/// * `None` - If the header doesn't use the Bearer scheme or is invalid
pub fn extract_access_key_from_header(header_value: &HeaderValue) -> Option<String> {
    let header_str = header_value.to_str().ok()?;
    if header_str.starts_with("Bearer ") {
        Some(header_str[7..].to_string())
    } else {
        None
    }
}

/// Validates an access key against the configured access keys
///
/// # Arguments
///
/// * `access_key` - The access key to validate
/// * `configured_keys` - The list of configured access keys
///
/// # Returns
///
/// * `Ok(ValidatedAccessKey)` - If the access key is valid
/// * `Err(AuthorizationError::Unauthorized)` - If the access key is invalid
pub fn validate_access_key(
    access_key: &str,
    configured_keys: &[String],
) -> Result<ValidatedAccessKey, AuthorizationError> {
    if configured_keys.contains(&access_key.to_string()) {
        Ok(ValidatedAccessKey {
            key: access_key.to_string(),
        })
    } else {
        Err(AuthorizationError::Unauthorized)
    }
}

/// Authorization middleware that validates access keys
///
/// This middleware only enforces authorization when the server is configured
///
/// # Arguments
///
/// * `request` - The incoming request
/// * `next` - The next middleware in the chain
///
/// # Returns
///
/// * `Response` - The response from the next middleware or an error response

pub async fn authorization_middleware(
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    // Get the state from the request extensions
    let state = request.extensions().get::<ProxyState>();
    
    // If we can't get the state, allow the request to proceed
    let state = match state {
        Some(s) => s,
        None => return Ok(next.run(request).await),
    };
    
    // Get the configuration
    let config = state.config.read().unwrap();
    
    // Only enforce authorization when listening on "0.0.0.0"
    if config.host != "0.0.0.0" {
        drop(config); // Release the read lock
        return Ok(next.run(request).await);
    }
    
    // If no access keys are configured, allow all requests
    if config.access_keys.is_empty() {
        drop(config); // Release the read lock
        return Ok(next.run(request).await);
    }
    
    // Clone the access keys for validation (to release the lock)
    let access_keys = config.access_keys.clone();
    drop(config); // Release the read lock
    
    // Extract the Authorization header
    let headers = request.headers();
    let auth_header = headers.get(AUTHORIZATION);
    
    // Check if Authorization header is missing
    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(AuthorizationError::MissingAuthorizationHeader.into_response()),
    };
    
    // Extract access key from header
    let access_key = match extract_access_key_from_header(auth_header) {
        Some(key) => key,
        None => return Err(AuthorizationError::InvalidAuthorizationScheme.into_response()),
    };
    
    // Validate access key
    match validate_access_key(&access_key, &access_keys) {
        Ok(_) => return Ok(next.run(request).await),
        Err(error) => return Err(error.into_response()),
    };    
}