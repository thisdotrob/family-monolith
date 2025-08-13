Add auth guard + hide introspection.

Tasks
1. Create `auth/guard.rs` implementing `async_graphql::extensions::Extension` which should
   Reject when `ctx.data_opt::<Claims>()` is None. See the docs for `Extension` here:
   https://docs.rs/async-graphql/latest/async_graphql/extensions/trait.Extension.html

2. In `graphql/mod.rs` builder chain `.extension(AuthGuard)` and call `.disable_introspection()`.

3. Add Axum middleware extracting JWT from `Authorization: Bearer` header, decoding, and inserting `Claims` into request extensions for authenticated routes.

4. Tests: anonymous query for `__schema` returns 403.

Commit message: "feat(sec): GraphQL auth guard & introspection disabled"
