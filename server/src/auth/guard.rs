use async_graphql::extensions::{Extension, ExtensionContext, NextRequest};
use async_graphql::{Response, ServerError};
use std::sync::Arc;

use crate::auth::Claims;

#[derive(Default)]
pub struct AuthGuard;

impl async_graphql::extensions::ExtensionFactory for AuthGuard {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthGuard)
    }
}

impl Extension for AuthGuard {
    fn request<'life0, 'life1, 'life2, 'life3, 'async_trait>(
        &'life0 self,
        ctx: &'life1 ExtensionContext<'life2>,
        next: NextRequest<'life3>,
    ) -> async_graphql::futures_util::future::BoxFuture<'async_trait, Response>
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
    {
        Box::pin(async move {
            if ctx.data_opt::<Arc<Claims>>().is_none() {
                dbg!("authentication required error in authguard");
                return Response::from_errors(vec![ServerError::new(
                    "Authentication required",
                    None,
                )]);
            }

            next.run(ctx).await
        })
    }
}
