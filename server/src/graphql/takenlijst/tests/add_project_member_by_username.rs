// Placeholder unit test for takenlijst/add_project_member_by_username resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::mutations::add_project_member_by_username::AddProjectMemberByUsernameMutation;

    #[tokio::test]
    async fn compiles_and_links_add_project_member_by_username() {
        let _ = AddProjectMemberByUsernameMutation::default();
        assert!(true);
    }
}
