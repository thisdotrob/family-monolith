use rand::Rng;
use std::time::Duration;

/// Verifies if the provided password matches the stored password
///
/// This is a simple plaintext comparison for now
pub async fn verify(stored: &str, provided: &str) -> bool {
    let matches = stored == provided;
    if !matches {
        // Generate the random number before the await point
        let delay_ms = rand::thread_rng().gen_range(200..=500);
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }
    matches
}
