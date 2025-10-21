// Placeholder unit test for takenlijst/rename_project resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::mutations::rename_project::RenameProjectMutation;

    #[tokio::test]
    async fn compiles_and_links_rename_project() {
        let _ = RenameProjectMutation::default();
        assert!(true);
    }
}
