#[macro_use]
extern crate if_chain;

use worker::{
    event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Env, Request, Response, Result, Router,
};

mod service;
mod utils;

use service::Service;

///
/// # Errors
/// This may result in an Err if a Response is invalid, or if any of the services result in an Err.
///
#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    Router::new()
        .post_async("/*request", |ctx, _| async move {
            Service::from(ctx).await.map_or_else(Ok, Service::response)
        })
        .run(req, env)
        .await
}
