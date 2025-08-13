use crate::{config, graphql};
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Request};
use axum::http::{HeaderMap, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::{
    Json, Router,
    extract::State,
    response::IntoResponse,
    routing::{get, post},
};
use axum_server;
use axum_server::tls_rustls::RustlsConfig;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::services::ServeDir;
use tower_http::trace::{self, TraceLayer};
use tracing::Span;

pub mod logging;
pub mod rate_limit;

#[derive(Clone)]
struct AppState {
    unauthenticated_schema: graphql::UnauthenticatedSchema,
    authenticated_schema: graphql::AuthenticatedSchema,
    pool: SqlitePool,
}

pub async fn run(port: u16) {
    logging::init();

    let pool = match crate::db::init(crate::config::DB_PATH).await {
        Ok(pool) => pool,
        Err(e) => {
            tracing::error!("Failed to initialize database: {}", e);
            return;
        }
    };

    let unauthenticated_schema = graphql::build_unauthenticated(pool.clone());
    let authenticated_schema = graphql::build_authenticated(pool.clone());

    let app_state = AppState {
        unauthenticated_schema,
        authenticated_schema,
        pool: pool.clone(),
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::POST])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::AUTHORIZATION,
        ])
        .max_age(std::time::Duration::from_secs(3600));

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/v1/healthz", get(health_check))
        .route("/v1/version", get(version))
        .route(
            "/v1/graphql/auth",
            post(graphql_unauthenticated_handler)
                .layer(middleware::from_fn(rate_limit::login_rate_limit)),
        )
        .route("/v1/graphql/app", post(graphql_authenticated_handler))
        .layer(middleware::from_fn(jwt_middleware))
        .layer(middleware::from_fn(content_type_middleware))
        .with_state(app_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_response(
                    |response: &axum::response::Response, latency: Duration, _span: &Span| {
                        let status = response.status();
                        let remote_addr = response
                            .extensions()
                            .get::<ConnectInfo<SocketAddr>>()
                            .map(|ci| ci.0.to_string())
                            .unwrap_or_else(|| "unknown".to_string());

                        tracing::info!(
                            "remote_addr = {}, status = {}, latency = {:?}",
                            remote_addr,
                            status,
                            latency
                        );
                    },
                ),
        )
        .layer(cors)
        .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024)); // 1MB limit

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    tracing::info!("listening on {}", addr);

    let cert_path = PathBuf::from(config::cert_path());
    let key_path = PathBuf::from(config::key_path());

    if cert_path.exists() && key_path.exists() {
        let config = match RustlsConfig::from_pem_file(cert_path, key_path).await {
            Ok(config) => config,
            Err(e) => {
                tracing::error!("Failed to create TLS config: {}", e);
                return;
            }
        };
        tracing::info!("TLS enabled");
        let server = axum_server::bind_rustls(addr, config)
            .serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    tracing::error!("server error: {}", e);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Shutting down gracefully");
                pool.close().await;
            }
        }
    } else {
        tracing::warn!("TLS certificates not found, starting in HTTP mode");
        let server =
            axum_server::bind(addr).serve(app.into_make_service_with_connect_info::<SocketAddr>());

        tokio::select! {
            result = server => {
                if let Err(e) = result {
                    tracing::error!("server error: {}", e);
                }
            }
            _ = tokio::signal::ctrl_c() => {
                tracing::info!("Shutting down gracefully");
                pool.close().await;
            }
        }
    }
}

async fn health_check(State(state): State<AppState>) -> StatusCode {
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn version() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "version": env!("CARGO_PKG_VERSION") }))
}

async fn content_type_middleware(request: Request, next: Next) -> impl IntoResponse {
    // Skip content-type check for health check, version endpoints and GET requests
    let path = request.uri().path();
    if path == "/v1/healthz" || path == "/v1/version" || request.method() == Method::GET {
        return next.run(request).await;
    }

    // Check content-type header
    if let Some(content_type) = request.headers().get("content-type") {
        if let Ok(content_type_str) = content_type.to_str() {
            if content_type_str.starts_with("application/json") {
                return next.run(request).await;
            }
        }
    }

    // Return 415 Unsupported Media Type if content-type is not application/json
    (
        StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "Content-Type must be application/json",
    )
        .into_response()
}

async fn jwt_middleware(headers: HeaderMap, request: Request, next: Next) -> impl IntoResponse {
    // Extract the Authorization header
    let auth_header = headers.get("Authorization");

    if let Some(auth_value) = auth_header {
        if let Ok(auth_str) = auth_value.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Skip "Bearer " prefix

                // Try to decode the JWT
                match crate::auth::decode(token) {
                    Ok(claims) => {
                        // Add claims to request extensions
                        let mut req = request;
                        req.extensions_mut().insert(Arc::new(claims));
                        return next.run(req).await;
                    }
                    Err(err) => {
                        tracing::debug!("JWT validation failed: {}", err);
                        // Continue without claims - the AuthGuard will handle rejection if needed
                    }
                }
            }
        }
    }

    // Continue the middleware chain without claims
    next.run(request).await
}

async fn graphql_unauthenticated_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let query = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UTF-8").into_response(),
    };

    let request = match serde_json::from_str::<async_graphql::Request>(&query) {
        Ok(req) => req,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid GraphQL request").into_response(),
    };

    let response = state.unauthenticated_schema.execute(request).await;

    (StatusCode::OK, Json(response)).into_response()
}

async fn graphql_authenticated_handler(
    State(state): State<AppState>,
    _headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let query = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UTF-8").into_response(),
    };

    let request = match serde_json::from_str::<async_graphql::Request>(&query) {
        Ok(req) => req,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid GraphQL request").into_response(),
    };

    let response = state.authenticated_schema.execute(request).await;

    (StatusCode::OK, Json(response)).into_response()
}

/*
MANUAL TEST

1. Run the server:
cargo run --bin dev

2. Make a request to the health check endpoint:
curl -v http://127.0.0.1:4173/v1/healthz

3. Check the logs for a line similar to this:
{"timestamp":"...","level":"INFO","fields":{"message":"request","remote_addr":"127.0.0.1:...", "method":"GET","path":"/v1/healthz","status":"200 OK","latency":"..."},"target":"...","line":...}
*/
