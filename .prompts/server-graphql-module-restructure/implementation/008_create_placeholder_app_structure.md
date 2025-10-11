# Ticket 008: Create Placeholder App Structure

## ID: 008
**Dependencies**: 001, 002, 006
**Parallel-safe**: Yes (can work in parallel with takenlijst restructuring)

## Objective
Create the placeholder app directory structure to demonstrate the multi-app organization pattern and provide a template for future apps.

## Tasks
1. Create `server/src/graphql/placeholder/` directory structure
2. Create basic placeholder resolvers following the file-per-resolver pattern
3. Implement `PlaceholderQuery` and `PlaceholderMutation` structs
4. Integrate placeholder app into main schema building
5. Create placeholder-specific tests structure

## Directory Structure to Create
```
server/src/graphql/placeholder/
├── mod.rs                    # Module organization and exports
├── tests/                    # Placeholder app tests
│   └── mod.rs               # Test module organization
├── hello.rs                 # Simple hello query resolver
└── echo.rs                  # Simple echo mutation resolver
```

## Implementation Details

### Basic Resolvers
Create simple example resolvers to demonstrate the pattern:

#### `placeholder/hello.rs`:
```rust
use async_graphql::{Context, Object};

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
```

#### `placeholder/echo.rs`:
```rust
use async_graphql::{Context, Object, InputObject, SimpleObject};

#[derive(InputObject)]
pub struct EchoInput {
    pub message: String,
}

#[derive(SimpleObject)]
pub struct EchoPayload {
    pub echo: String,
    pub success: bool,
}

#[derive(Default)]
pub struct EchoMutation;

#[Object]
impl EchoMutation {
    async fn echo(&self, _ctx: &Context<'_>, input: EchoInput) -> EchoPayload {
        EchoPayload {
            echo: format!("Echo: {}", input.message),
            success: true,
        }
    }
}
```

### Module Organization (`placeholder/mod.rs`):
```rust
use async_graphql::MergedObject;

mod hello;
mod echo;

#[cfg(test)]
mod tests;

pub use hello::HelloQuery;
pub use echo::EchoMutation;

#[derive(MergedObject, Default)]
pub struct PlaceholderQuery(HelloQuery);

#[derive(MergedObject, Default)]
pub struct PlaceholderMutation(EchoMutation);
```

### Integration with Main Schema
Update main `mod.rs` to include placeholder app:
```rust
// Add to mod.rs
mod placeholder;
use placeholder::{PlaceholderQuery, PlaceholderMutation};

#[derive(MergedObject, Default)]
pub struct QueryRoot(SharedQuery, TakenlijstQuery, PlaceholderQuery);

#[derive(MergedObject, Default)]
pub struct CombinedMutation(SharedMutation, TakenlijstMutation, PlaceholderMutation);
```

### Test Structure (`placeholder/tests/mod.rs`):
```rust
#[cfg(test)]
mod tests {
    use crate::graphql::placeholder::{HelloQuery, EchoMutation, EchoInput};
    use async_graphql::{Context, Object};

    #[tokio::test]
    async fn test_hello_query() {
        // Basic test for hello query
    }

    #[tokio::test]
    async fn test_echo_mutation() {
        // Basic test for echo mutation
    }
}
```

## Verification
- Code compiles successfully
- Placeholder resolvers are accessible in GraphQL schema
- `hello` query and `echo` mutation work correctly
- Tests pass for placeholder functionality
- Demonstrates clean app organization pattern
- Provides template for future app modules

## Files Created
- `server/src/graphql/placeholder/mod.rs`
- `server/src/graphql/placeholder/hello.rs`
- `server/src/graphql/placeholder/echo.rs`
- `server/src/graphql/placeholder/tests/mod.rs`

## Files Modified
- `server/src/graphql/mod.rs` - Add placeholder app integration

## Future Use
This structure serves as a template for adding new apps:
1. Create `server/src/graphql/{newapp}/` directory
2. Follow the same file-per-resolver pattern
3. Create `{NewApp}Query` and `{NewApp}Mutation` structs
4. Add to main schema building in `mod.rs`
5. Include appropriate tests structure