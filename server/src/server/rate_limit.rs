use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use serde_json::Value;
use std::{
    net::SocketAddr,
    time::{Duration, Instant},
};

// Constants for rate limiting
const MAX_LOGIN_ATTEMPTS: usize = 10;
const RATE_LIMIT_WINDOW_SECS: u64 = 60;

// Global rate limiter state using DashMap for thread-safe access
static LOGIN_ATTEMPTS: Lazy<DashMap<String, Vec<Instant>>> = Lazy::new(DashMap::new);

// Middleware for rate limiting login attempts
pub async fn login_rate_limit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request<Body>,
    next: Next,
) -> Response {
    // Extract the request body to check if it's a login operation
    let (parts, body) = request.into_parts();
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    // Try to parse as JSON to check if it's a login operation
    let is_login_operation = match serde_json::from_slice::<Value>(&bytes) {
        Ok(json) => {
            // Check if this is a login mutation
            let query = json.get("query").and_then(Value::as_str).unwrap_or("");
            query.contains("login") && query.contains("mutation")
        }
        Err(_) => false,
    };

    // If it's not a login operation, proceed normally
    if !is_login_operation {
        let request = Request::from_parts(parts, Body::from(bytes));
        return next.run(request).await;
    }

    // For login operations, apply rate limiting
    let ip = addr.ip().to_string();
    let now = Instant::now();
    let window_start = now - Duration::from_secs(RATE_LIMIT_WINDOW_SECS);

    // Clean up old attempts and check rate limit
    let mut exceeded_limit = false;

    // Use entry API correctly
    if let Some(mut attempts) = LOGIN_ATTEMPTS.get_mut(&ip) {
        // Remove attempts outside the current window
        attempts.retain(|time| *time > window_start);

        // Check if we've exceeded the limit
        if attempts.len() >= MAX_LOGIN_ATTEMPTS {
            exceeded_limit = true;
        } else {
            // Add the current attempt
            attempts.push(now);
        }
    } else {
        // If this is a new entry, add the current attempt
        LOGIN_ATTEMPTS.insert(ip.clone(), vec![now]);
    }

    // If rate limit exceeded, return 429 response
    if exceeded_limit {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            [("Retry-After", "60")],
            "Too many login attempts. Please try again later.",
        )
            .into_response();
    }

    // Proceed with the request
    let request = Request::from_parts(parts, Body::from(bytes));
    next.run(request).await
}
