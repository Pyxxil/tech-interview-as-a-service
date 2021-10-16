#[macro_use]
extern crate if_chain;

use worker::{
    event, wasm_bindgen, wasm_bindgen_futures, worker_sys, Env, Method, Request, Response, Result,
    Router,
};

mod service;
mod utils;

use service::Service;

use crate::service::fizzbuzz::{self, FizzBuzz};
use crate::service::sort::{self, Sort};

fn not_found() -> Result<Response> {
    Response::error("Not found", 404)
}

async fn handle(mut ctx: Request) -> Result<Response> {
    if_chain! {
        if let Ok(url) = ctx.url();
        if let Some(mut path) = url.path_segments();
        if let Some(service) = path.next();
        if let Some(query) = url.query();

        then {
            match (ctx.method(), service.to_ascii_lowercase().as_str()) {
                (Method::Post, fizzbuzz::NAME) => FizzBuzz::create(None, query).map_or_else(Ok, Service::response),
                (Method::Get, fizzbuzz::NAME) =>
                    Ok(FizzBuzz::help(Some((
                        "Try sending a POST request to this endpoint".to_string(),
                        200,
                    )))),

                (Method::Post, sort::NAME) => Sort::create(Some(ctx.json().await), query).map_or_else(Ok, Service::response),
                (Method::Get, sort::NAME) => Ok(Sort::help(Some((
                    "Try sending a POST request to this endpoint".to_string(),
                    200,
                )))),

                _ => not_found()
            }
        } else {
            not_found()
        }
    }
}

///
/// # Errors
/// This may result in an Err if a Response is invalid, or if any of the services result in an Err.
///
#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    Router::new()
        .get_async("/*request", |ctx, _| async move { handle(ctx).await })
        .post_async("/*request", |ctx, _| async move { handle(ctx).await })
        .run(req, env)
        .await
}
