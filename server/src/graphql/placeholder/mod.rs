use async_graphql::MergedObject;

mod echo;
mod hello;

#[cfg(test)]
mod tests;

pub use echo::EchoMutation;
pub use hello::HelloQuery;

#[derive(MergedObject, Default)]
pub struct PlaceholderQuery(HelloQuery);

#[derive(MergedObject, Default)]
pub struct PlaceholderMutation(EchoMutation);
