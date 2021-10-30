use serde::Deserialize;
use worker::{Method, Request, Response, Result};

use crate::{
    services::{fizzbuzz::FizzBuzz, index::Index, sort::Sort},
    traits::Service,
};

pub mod fizzbuzz;
pub mod index;
pub mod sort;

pub const SERVICES: [&str; 3] = [fizzbuzz::NAME, sort::NAME, index::NAME];

///
/// Serve the request
///
async fn serve<T: Service + for<'de> Deserialize<'de>>(mut ctx: Request) -> Result<Response> {
    if ctx.method() != Method::Post {
        T::help()
    } else if ctx
        .headers()
        .get("Content-Type")
        .contains(&Some(String::from("application/json")))
    {
        ctx.json()
            .await
            .map(T::response)
            .unwrap_or_else(|err| T::error(&err.to_string(), 400))
    } else {
        T::error(
            "Invalid Content-Type header; Expected 'application/json'",
            415,
        )
    }
}

///
/// Transform the Request's URL into the actual service to deal with
/// it, e.g. `${URL}/fizzbuzz` into a `Fizzbuzz` service.
///
pub(crate) async fn handle(ctx: Request) -> Result<Response> {
    let url = ctx.url().unwrap();
    let service = url.path_segments().unwrap().next().unwrap();

    match service.to_ascii_lowercase().as_str() {
        fizzbuzz::NAME => serve::<FizzBuzz>(ctx).await,
        sort::NAME => serve::<Sort>(ctx).await,
        index::NAME => serve::<Index>(ctx).await,

        // This is handled by the `Router` class, such that the
        // only time a string we don't handle above could be seen
        // here is if a service has forgotten to be added.
        _ => Response::error(format!("`{}` Not Found", service), 404),
    }
}
