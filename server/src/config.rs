//! Configuration constants for the application

/// Path to the SQLite database file
pub const DB_PATH: &str = "./blobfishapp.sqlite";

/// JWT secret key - should be changed in production
pub const JWT_SECRET: &str = "CHANGE_ME_AT_DEPLOY";

use std::env;

/// Get the path to the fullchain.pem file
pub fn cert_path() -> String {
    env::var("TLS_CERT_PATH").unwrap_or_else(|_| {
        "/etc/letsencrypt/live/blobfishapp.duckdns.org/fullchain.pem".to_string()
    })
}

/// Get the path to the privkey.pem file
pub fn key_path() -> String {
    env::var("TLS_KEY_PATH")
        .unwrap_or_else(|_| "/etc/letsencrypt/live/blobfishapp.duckdns.org/privkey.pem".to_string())
}
