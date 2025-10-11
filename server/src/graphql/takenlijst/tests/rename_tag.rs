// Placeholder unit test for takenlijst/rename_tag resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::rename_tag::RenameTagMutation;

    #[tokio::test]
    async fn compiles_and_links_rename_tag() {
        let _ = RenameTagMutation::default();
        assert!(true);
    }
}
