use worker::{event, Context, Env, Request, Response, Result, Router};

mod services;
mod traits;
mod utils;

use traits::Service;

use crate::services::{handle, SERVICES};

///
/// # Errors
/// This may result in an Err if a Response is invalid, or if any of the services result in an Err.
///
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    utils::set_panic_hook();

    SERVICES
        .iter()
        .fold(Router::new(), |router, service| {
            router
                .get_async(service, |ctx, _| async move { handle(ctx).await })
                .post_async(service, |ctx, _| async move { handle(ctx).await })
        })
        .run(req, env)
        .await
}
