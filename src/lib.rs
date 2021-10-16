#![feature(option_result_contains)]

use serde::Deserialize;
use worker::{
    event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Env, Method, Request, Response, Result,
    Router,
};

mod services;
mod traits;
mod utils;

use traits::Service;

use crate::services::fizzbuzz::{self, FizzBuzz};
use crate::services::sort::{self, Sort};

///
/// Serve the request
///
async fn serve<T: Service + for<'de> Deserialize<'de>>(mut ctx: Request) -> Result<Response> {
    if ctx.method() != Method::Post {
        return T::help();
    }

    if ctx
        .headers()
        .get("Content-Type")
        .contains(&Some(String::from("application/json")))
    {
        ctx.json()
            .await
            .map_or_else(|err| T::error(&err.to_string(), 400), T::response)
    } else {
        T::error(
            "Invalid Content-Type header; Expected 'application/json'",
            415,
        )
    }
}

///
/// Handle the request based on it's context.
///
///
async fn handle(ctx: Request) -> Result<Response> {
    let url = ctx.url().unwrap();
    let service = url.path_segments().unwrap().next().unwrap();

    match service.to_ascii_lowercase().as_str() {
        fizzbuzz::NAME => serve::<FizzBuzz>(ctx).await,
        sort::NAME => serve::<Sort>(ctx).await,
        _ => unreachable!(),
    }
}

///
/// # Errors
/// This may result in an Err if a Response is invalid, or if any of the services result in an Err.
///
#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    [fizzbuzz::NAME, sort::NAME]
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
