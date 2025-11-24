// middleware that shows how to consume the request body upfront
async fn print_request_body(request: Request<Body>, next: Next) -> Result<impl IntoResponse, Response> {
    let request = buffer_request_body(request).await?;

    Ok(next.run(request).await)
}

async fn buffer_request_body(request: Request<Body>) -> Result<Request<Body>, Response> {
    let (parts, body) = request.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    log_bytes(bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn log_bytes(bytes: Bytes) {
    log::debug!("{}", String::from_utf8(bytes.to_vec()).unwrap());
}

async fn log_axum_response(response: Response) -> Response {
    log::debug!("Response Status: {}", response.status());
    log::debug!("Response Version: {:?}", response.version());
    log::debug!("Response Headers:");
    for (name, value) in response.headers() {
        log::debug!("  {}: {:?}", name, value);
    }

    let (parts, body) = response.into_parts();


    let bytes = body.collect()
        .await
        .unwrap()
        .to_bytes();

    log_bytes(bytes.clone());

    Response::from_parts(parts, Body::from(bytes))
}

async fn log_response_metadata(response: &reqwest::Response) {
    log::debug!("Response Status: {}", response.status());
    log::debug!("Response Version: {:?}", response.version());
    log::debug!("Response Headers:");
    for (name, value) in response.headers() {
        log::debug!("  {}: {:?}", name, value);
    }
    if let Some(content_length) = response.content_length() {
        log::debug!("Content Length: {}", content_length);
    }
}

fn log_header(headers: &HeaderMap) {
    log::debug!("Headers:");
    for (name, value) in headers {
        log::debug!("  {}: {:?}", name, value);
    }
}

fn log_request(request: &reqwest::Request) {
    log::debug!("--- Request Details ---");
    log::debug!("Method: {}", request.method());
    log::debug!("URL: {}", request.url());

    // Log headers
    log::debug!("Headers:");
    for (name, value) in request.headers() {
        log::debug!("  {}: {:?}", name, value);
    }

    // Attempt to log the body. This is more complex as the body might be a stream.
    // For simple cases where the body is already materialized (e.g., from a string or bytes),
    // you might be able to clone and read it. For streaming bodies, this would require
    // consuming the stream, which might not be desirable before sending the request.
    // The following example assumes a readily available body.
    // In a real-world scenario with streaming bodies, you might need to use a middleware
    // or a custom approach to capture and log the body content without consuming it prematurely.

    // Note: Accessing the body directly from a `&Request` might not be possible
    // if it's a `Body` enum variant that's not easily cloned or inspected.
    // If you have a `RequestBuilder` before building the `Request`, you might have
    // more direct access to the body data before it's wrapped.

    // Example of how you might *try* to get the body, but this is not generally safe
    // for all `Body` types without consuming or cloning.
    // If you're building the request and have the body data before `send()`,
    // it's better to log it then.
    //
    // if let Some(body) = request.body() {
    //     // This part is highly dependent on the Body type and might require
    //     // a custom solution or a library like `reqwest-middleware` with a tracing layer.
    //     // For simple cases like `Body::from("string")`, you might be able to get bytes.
    //     // However, for `Body::from(File)` or other streams, it's not straightforward.
    //     println!("Body: (content not easily loggable without consuming stream)");
    // }

    log::debug!("-----------------------");
}

fn get_access_token_from_bearer(header_value: &str) -> Option<String> {
    if header_value.starts_with("Bearer ") {
        Some(String::from(&header_value[7..]))
    } else {
        None
    }
}
