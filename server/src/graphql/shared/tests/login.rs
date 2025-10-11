// Unit tests for shared/login resolver

#[cfg(test)]
mod tests {
    // Import types to validate module paths are correct after restructure
    use crate::graphql::types::{login_input::LoginInput, login_payload::LoginPayload};

    // Import resolver type (ensures module export correctness)
    use crate::graphql::shared::LoginMutation;

    #[tokio::test]
    async fn compiles_and_links_login_resolver_types() {
        // This is a placeholder compilation test to ensure the new module structure is wired
        // correctly. Full behavioral tests live in integration tests.
        let _ = LoginMutation::default();
        let _ = LoginInput {
            username: "u".into(),
            password: "p".into(),
        };
        let _ = LoginPayload {
            success: false,
            token: None,
            refresh_token: None,
            errors: vec![],
        };
        assert!(true);
    }
}
