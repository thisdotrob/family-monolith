// Placeholder unit test for takenlijst/delete_saved_view resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::DeleteSavedViewMutation;

    #[tokio::test]
    async fn compiles_and_links_delete_saved_view() {
        let _ = DeleteSavedViewMutation::default();
        assert!(true);
    }
}
