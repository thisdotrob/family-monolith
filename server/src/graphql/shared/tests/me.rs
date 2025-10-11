// Unit tests for shared/me resolver

#[cfg(test)]
mod tests {
    use crate::graphql::shared::MeQuery;

    #[tokio::test]
    async fn compiles_and_links_me_query() {
        let _ = MeQuery::default();
        assert!(true);
    }
}
