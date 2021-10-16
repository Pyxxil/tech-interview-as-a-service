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
pub(crate) async fn serve<T: Service + for<'de> Deserialize<'de>>(
    mut ctx: Request,
) -> Result<Response> {
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
pub(crate) async fn handle(ctx: Request) -> Result<Response> {
    let url = ctx.url().unwrap();
    let service = url.path_segments().unwrap().next().unwrap();

    match service.to_ascii_lowercase().as_str() {
        fizzbuzz::NAME => serve::<FizzBuzz>(ctx).await,
        sort::NAME => serve::<Sort>(ctx).await,
        index::NAME => serve::<Index>(ctx).await,
        _ => unreachable!(),
    }
}
