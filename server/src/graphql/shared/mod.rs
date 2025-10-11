use async_graphql::MergedObject;

mod login;
mod logout;
mod me;
mod refresh_token;

pub use login::LoginMutation;
pub use logout::LogoutMutation;
pub use me::MeQuery;
pub use refresh_token::RefreshTokenMutation;

#[derive(MergedObject, Default)]
pub struct SharedMutation(LoginMutation, RefreshTokenMutation, LogoutMutation);

#[derive(MergedObject, Default)]
pub struct SharedQuery(MeQuery);
