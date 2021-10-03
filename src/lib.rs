#[macro_use]
extern crate if_chain;

use worker::*;

mod service;
mod utils;

use service::Service;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    utils::set_panic_hook();

    Router::new()
        .get("/*request", |ctx, _| {
            Service::from(ctx.url()).map_or_else(Ok, Service::response)
        })
        .run(req, env)
        .await
}
