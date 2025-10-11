// Unit tests for shared/refresh_token resolver

#[cfg(test)]
mod tests {
    use crate::graphql::shared::RefreshTokenMutation;
    use crate::graphql::types::{refresh_input::RefreshInput, refresh_payload::RefreshPayload};

    #[tokio::test]
    async fn compiles_and_links_refresh_token_resolver_types() {
        let _ = RefreshTokenMutation::default();
        let _ = RefreshInput {
            refresh_token: "r".into(),
        };
        let _ = RefreshPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec![],
        };
        assert!(true);
    }
}
