use std::sync::Arc;
use axum::{
    middleware::{Next, from_fn_with_state},
    Router,
    body::{Body, Bytes},
    extract::State,
    http::{Request, HeaderMap, StatusCode, Uri, header::AUTHORIZATION},
    response::{IntoResponse, Response},
    routing::post,
};
use reqwest::Client;
use futures_util::stream::StreamExt;
use http_body_util::BodyExt;

mod models;
mod logger;
mod key_manager;
mod state;
mod middleware;

use models::config::Config as cfg;
use state::State as ProxyState;
use key_manager::KeyManager;
use middleware::authorization;

async fn fallback(uri: Uri) -> (StatusCode, String) {
    log::error!("fallback url: {}", uri);
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

#[tokio::main]
async fn main() {
    let config = cfg::instance();
    logger::configure_logger();
    let address = format!("{}:{}", config.host, config.port);
    let km = KeyManager::new(config.api_keys.clone());

    let state =  ProxyState::new(config.clone(), km).await;
    let app = Router::new()
    .route("/chat/completions", post(chat_completions_handler))
    .layer(
        from_fn_with_state(
        Arc::new(config).clone(),
        auth_middleware))
    .fallback(fallback)
    .with_state(state);

    log::info!("listening on {address}");

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn auth_middleware(
    State(config): State<Arc<cfg>>,
    request: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, Response> {
    // Only enforce authorization when listening on "0.0.0.0"
    if config.host != "0.0.0.0" {
        return Ok(next.run(request).await);
    }
    
    // If no access keys are configured, allow all requests
    if config.access_keys.is_empty() {
        return Ok(next.run(request).await);
    }
    
    // Clone the access keys for validation (to release the lock)
    let access_keys = config.access_keys.clone();
    
    // Extract the Authorization header
    let headers = request.headers();
    let auth_header = headers.get(AUTHORIZATION);
    
    // Check if Authorization header is missing
    let auth_header = match auth_header {
        Some(header) => header,
        None => return Err(models::AuthorizationError::MissingAuthorizationHeader.into_response()),
    };
    
    // Extract access key from header
    let access_key = match authorization::extract_access_key_from_header(auth_header) {
        Some(key) => key,
        None => return Err(models::AuthorizationError::InvalidAuthorizationScheme.into_response()),
    };
    
    // Validate access key
    match authorization::validate_access_key(&access_key, &access_keys) {
        Ok(_) => return Ok(next.run(request).await),
        Err(error) => return Err(error.into_response()),
    };    
}

async fn chat_completions_handler(
    State(state): State<ProxyState>,
    mut headers: HeaderMap,
    request: axum::http::Request<axum::body::Body>,
) -> impl IntoResponse {
    log::info!("/chat/completions");

    let client = Client::new();

    headers.remove("authorization");
    headers.remove("host");
    let api_key = state.key_manager.write().unwrap().get_key();
    let auth_value = format!("Bearer {}", api_key);
    headers.append("authorization", auth_value.parse().unwrap());

    let base_url = state.config.read().unwrap().base_url.clone();
    let collected_body = match  request.into_body().collect().await {
        Ok(collection) => collection.to_bytes(),
        Err(_) => Bytes::new(),
    };

    let req = client
        .post(format!("{base_url}/chat/completions",))
        .headers(headers)
        .body(collected_body);
    let data = req.build().unwrap();
    let res = match client.execute(data).await {
        Ok(res) => res,
        Err(e) => {
            log::error!("{}", e);

            let mut axum_response = Response::new(Body::from(e.to_string()));
            *axum_response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

            return axum_response;
        },
    }; 
    let is_stream = if let Some(header_value) = res.headers().get("content-type") {
        match header_value.to_str() {
            Ok(s) => s == "text/event-stream",
            Err(_) => false,
        }
    } else { false };
    let headers = res.headers().clone(); 
    let status = res.status();

    log::info!("---!!! response status: {}", status.as_str());

    if !status.is_success() {   
        if status == StatusCode::TOO_MANY_REQUESTS {
            // switch to next key
            state.key_manager.write().unwrap().switch_key();
        }
    }

    let body = if is_stream {
        log::debug!("--->>> stream response");
        let stream = async_stream::stream! {
            let mut stream = res.bytes_stream();

            while let Some(item) = stream.next().await {
                log::trace!("---+++ chank");
                yield Ok::<_, axum::Error>(item.unwrap());                
            }         
        };

        Body::from_stream(stream)        
    } else {
        log::debug!("--->>> single response");
        let body_bytes = res.bytes().await.unwrap();
        Body::from(body_bytes)
    };

    let mut axum_response = Response::new(body);
    *axum_response.status_mut() = status;
    *axum_response.headers_mut() = headers;

    axum_response
}
