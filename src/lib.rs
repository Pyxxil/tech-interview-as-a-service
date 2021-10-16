#![feature(option_result_contains)]

use worker::{
    event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Env, Request, Response, Result, Router,
};

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
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    SERVICES
        .iter()
        .fold(Router::new(), |router, service| {
            let service = format!("/{}", service);
            router
                .get_async(&service, |ctx, _| async move { handle(ctx).await })
                .post_async(&service, |ctx, _| async move { handle(ctx).await })
        })
        .run(req, env)
        .await
}
