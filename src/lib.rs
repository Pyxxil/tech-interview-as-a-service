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
            if let Ok(url) = ctx.url() {
                Ok(Service::new(url)
                    .map(|service| service.response().unwrap())
                    .unwrap_or_else(|e| e))
            } else {
                Response::error("Not Found", 404)
            }
        })
        .run(req, env)
        .await
}
