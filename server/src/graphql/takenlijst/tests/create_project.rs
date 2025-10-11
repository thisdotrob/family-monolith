// Placeholder unit test for takenlijst/create_project resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::create_project::CreateProjectMutation;

    #[tokio::test]
    async fn compiles_and_links_create_project() {
        let _ = CreateProjectMutation::default();
        assert!(true);
    }
}
