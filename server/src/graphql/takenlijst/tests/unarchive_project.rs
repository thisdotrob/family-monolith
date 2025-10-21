// Placeholder unit test for takenlijst/unarchive_project resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::mutations::unarchive_project::UnarchiveProjectMutation;

    #[tokio::test]
    async fn compiles_and_links_unarchive_project() {
        let _ = UnarchiveProjectMutation::default();
        assert!(true);
    }
}
