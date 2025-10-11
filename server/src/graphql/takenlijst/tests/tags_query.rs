// Placeholder unit test for takenlijst/tags_query

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::TagsQuery;

    #[tokio::test]
    async fn compiles_and_links_tags_query() {
        let _ = TagsQuery::default();
        assert!(true);
    }
}
