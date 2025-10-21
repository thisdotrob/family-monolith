// Placeholder unit test for takenlijst/archive_project resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::mutations::archive_project::ArchiveProjectMutation;

    #[tokio::test]
    async fn compiles_and_links_archive_project() {
        let _ = ArchiveProjectMutation::default();
        assert!(true);
    }
}
