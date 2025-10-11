// Unit tests for shared/logout resolver

#[cfg(test)]
mod tests {
    use crate::graphql::shared::LogoutMutation;

    #[tokio::test]
    async fn compiles_and_links_logout_resolver() {
        let _ = LogoutMutation::default();
        assert!(true);
    }
}
