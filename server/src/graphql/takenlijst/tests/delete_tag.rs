// Placeholder unit test for takenlijst/delete_tag resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::mutations::delete_tag::DeleteTagMutation;

    #[tokio::test]
    async fn compiles_and_links_delete_tag() {
        let _ = DeleteTagMutation::default();
        assert!(true);
    }
}
