use async_graphql::Object;

#[derive(Default)]
pub struct HelloQuery;

#[Object]
impl HelloQuery {
    async fn hello(&self, name: Option<String>) -> String {
        match name {
            Some(n) => format!("Hello, {}!", n),
            None => "Hello, World!".to_string(),
        }
    }
}
