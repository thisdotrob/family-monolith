Wire empty async-graphql schema.

Tasks
1. In `graphql/mod.rs`:
   ```rust
   use async_graphql::{Schema, EmptyQuery, EmptyMutation, EmptySubscription};

   pub type AppSchema = Schema<EmptyQuery, EmptyMutation, EmptySubscription>;

   pub fn build() -> AppSchema { Schema::build(EmptyQuery, EmptyMutation, EmptySubscription).finish() }
   ```

2. In `server/mod.rs` add route:
    ```
    use crate::graphql;
    let schema = graphql::build();
    let graphql_route = axum::routing::post("/v1/graphql").to(
        |schema: axum::extract::Extension<graphql::AppSchema>, req: async_graphql_axum::GraphQLRequest| async move {
            async_graphql_axum::GraphQLResponse(schema.execute(req.into_inner()).await)
        }
    );
    router.merge(graphql_route).layer(axum::Extension(schema));
    ```

3. Add dependency async-graphql-axum = "7.0".

4. Integration test: POST {query:"query { __typename }"} returns HTTP 200.

Commit message: "feat(gql): empty schema wired to /v1/graphql"
