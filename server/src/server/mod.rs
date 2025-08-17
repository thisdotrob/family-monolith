use crate::{config, graphql, AppError};
use crate::error_codes::ErrorCode;
use axum::body::Bytes;
use axum::extract::{ConnectInfo, Extension, Request};
use axum::http::{Method, StatusCode};
use axum::middleware::{self, Next};
use axum::{
    response::Response, Json, Router, extract::State, response::IntoResponse, routing::get,
    routing::post,
};
use axum_server;
use axum_server::tls_rustls::RustlsConfig;
use jsonwebtoken::errors::ErrorKind;
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
    schema: graphql::AppSchema,
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

    let schema = graphql::build(pool.clone());

    let app_state = AppState {
        schema,
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
            "/v1/graphql",
            post(graphql_unified_handler)
                .layer(middleware::from_fn(rate_limit::login_rate_limit)),
        )
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
        return next.run(request).await.into_response();
    }

    // Check content-type header
    if let Some(content_type) = request.headers().get("content-type") {
        if let Ok(content_type_str) = content_type.to_str() {
            if content_type_str.starts_with("application/json") {
                return next.run(request).await.into_response();
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

async fn jwt_middleware(request: Request, next: Next) -> Result<Response, AppError> {
    // Read body so we can decide whether this is an unauthenticated mutation (login/refresh)
    let (parts, body) = request.into_parts();
    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => Default::default(),
    };

    // Detect if mutation is one of the unauthenticated ones
    let mut is_unauth_mutation = false;
    if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        if let Some(query) = json.get("query").and_then(|v| v.as_str()) {
            let q = query;
            let is_mutation = q.contains("mutation");
            let is_login = q.contains("login");
            let is_refresh = q.contains("refresh_token") || q.contains("refreshToken");
            is_unauth_mutation = is_mutation && (is_login || is_refresh);
        }
    }

    // Reconstruct the request so downstream handlers can read the body
    let mut request = Request::from_parts(parts, axum::body::Body::from(bytes.clone()));

    // Now handle JWT if provided
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok());

    if let Some(auth_str) = auth_header {
        if auth_str.starts_with("Bearer ") {
            let token = &auth_str[7..];

            match crate::auth::decode(token) {
                Ok(claims) => {
                    request.extensions_mut().insert(Arc::new(claims));
                }
                Err(e) => {
                    tracing::debug!("JWT validation failed");
                    match e.kind() {
                        ErrorKind::ExpiredSignature => {
                            if !is_unauth_mutation {
                                // For expired tokens on authenticated operations, return 401 so the client can refresh.
                                return Err(AppError { code: ErrorCode::TokenExpired, msg: "token has expired".into() });
                            }
                            // Otherwise allow unauthenticated mutations to proceed without claims
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(next.run(request).await)
}

async fn graphql_unified_handler(
    State(state): State<AppState>,
    claims: Option<Extension<Arc<crate::auth::Claims>>>,
    body: Bytes,
) -> impl IntoResponse {
    dbg!("graphql_unified_handler");
    let query = match String::from_utf8(body.to_vec()) {
        Ok(text) => text,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UTF-8").into_response(),
    };

    let mut request = match serde_json::from_str::<async_graphql::Request>(&query) {
        Ok(req) => req,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid GraphQL request").into_response(),
    };

    if let Some(Extension(claims_data)) = claims {
        request = request.data(claims_data);
        dbg!("some claims");
    } else {
        dbg!("no claims");
    }

    let response = state.schema.execute(request).await;

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
