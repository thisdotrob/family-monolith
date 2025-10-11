// Placeholder unit test for takenlijst/create_recurring_series resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::CreateRecurringSeriesMutation;

    #[tokio::test]
    async fn compiles_and_links_create_recurring_series() {
        let _ = CreateRecurringSeriesMutation::default();
        assert!(true);
    }
}
