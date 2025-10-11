// Placeholder unit test for takenlijst/history_query

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::HistoryQuery;

    #[tokio::test]
    async fn compiles_and_links_history_query() {
        let _ = HistoryQuery::default();
        assert!(true);
    }
}
