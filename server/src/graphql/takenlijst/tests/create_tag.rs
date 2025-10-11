// Placeholder unit test for takenlijst/create_tag resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::create_tag::CreateTagMutation;

    #[tokio::test]
    async fn compiles_and_links_create_tag() {
        let _ = CreateTagMutation::default();
        assert!(true);
    }
}
