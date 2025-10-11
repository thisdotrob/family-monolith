# Ticket 002: Create Shared Directory Structure

## ID: 002
**Dependencies**: 001
**Parallel-safe**: No (depends on types)

## Objective
Create the `server/src/graphql/shared/` directory structure and extract authentication-related resolvers from existing files into individual resolver files.

## Tasks
1. Create `server/src/graphql/shared/` directory
2. Create `server/src/graphql/shared/tests/` directory for shared resolver tests
3. Extract resolvers from existing files:
   - `login` mutation from `unauthenticated.rs` → `shared/login.rs`
   - `refreshToken` mutation from `unauthenticated.rs` → `shared/refresh_token.rs`
   - `logout` mutation from `auth.rs` → `shared/logout.rs`
   - `me` query from `mod.rs` → `shared/me.rs`
4. Create `shared/mod.rs` with `SharedMutation` and `SharedQuery` structs using `MergedObject`

## Implementation Details

### Individual Resolver Files
Each resolver file should:
- Define its own struct (e.g., `LoginMutation`, `MeQuery`)
- Import required types from `crate::graphql::types`
- Contain only the single resolver function
- Use `#[derive(Default)]` and `#[Object]` patterns

### Example Structure for `shared/login.rs`:
```rust
use crate::graphql::types::{LoginInput, LoginPayload};
use async_graphql::{Context, Object};
use sqlx::SqlitePool;

#[derive(Default)]
pub struct LoginMutation;

#[Object]
impl LoginMutation {
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> LoginPayload {
        // Move existing login logic here
    }
}
```

### Shared Module Organization (`shared/mod.rs`):
```rust
use async_graphql::MergedObject;

mod login;
mod refresh_token;
mod logout;
mod me;

pub use login::LoginMutation;
pub use refresh_token::RefreshTokenMutation;
pub use logout::LogoutMutation;
pub use me::MeQuery;

#[derive(MergedObject, Default)]
pub struct SharedMutation(LoginMutation, RefreshTokenMutation, LogoutMutation);

#[derive(MergedObject, Default)]
pub struct SharedQuery(MeQuery);
```

## Verification
- Code compiles successfully
- Each resolver is in its own file
- `SharedMutation` and `SharedQuery` can be instantiated
- All authentication flows continue to work
- No changes to GraphQL schema structure

## Files Created
- `server/src/graphql/shared/mod.rs`
- `server/src/graphql/shared/login.rs`
- `server/src/graphql/shared/refresh_token.rs`
- `server/src/graphql/shared/logout.rs`
- `server/src/graphql/shared/me.rs`
- `server/src/graphql/shared/tests/` (directory)

## Files Modified
- `server/src/graphql/mod.rs` - Remove `me` query, update imports
- `server/src/graphql/unauthenticated.rs` - Remove login/refresh resolvers
- `server/src/graphql/auth.rs` - Remove logout resolver