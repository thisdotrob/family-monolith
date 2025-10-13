// Placeholder unit test for takenlijst/update_recurring_series resolver

#[cfg(test)]
mod tests {
    use crate::graphql::takenlijst::UpdateRecurringSeriesMutation;

    #[tokio::test]
    async fn compiles_and_links_update_recurring_series() {
        let _ = UpdateRecurringSeriesMutation::default();
        assert!(true);
    }
}
