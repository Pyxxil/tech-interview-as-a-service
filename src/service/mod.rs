use worker::{Request, Response, Result};

mod fizzbuzz;
use crate::service::fizzbuzz::FizzBuzz;
mod sort;
use crate::service::sort::Sort;

#[derive(Debug)]
pub enum Service {
    FizzBuzz(FizzBuzz),
    Sort(Sort),
    NotFound,
}

impl Service {
    pub async fn from(mut context: Request) -> std::result::Result<Service, Response> {
        if_chain! {
            if let Ok(url) = context.url();
            if let Some(mut path) = url.path_segments();
            if let Some(service) = path.next();

            then {
                return match service.to_ascii_lowercase().as_str() {
                    fizzbuzz::NAME => FizzBuzz::from(url.query()).map(Service::FizzBuzz),
                    sort::NAME => Sort::from(context.json().await, url.query()).map(Service::Sort),
                    _ => Ok(Service::NotFound)
                };
            }
        }

        Ok(Service::NotFound)
    }
}

impl Service {
    pub fn response(self) -> Result<Response> {
        match self {
            Service::FizzBuzz(s) => s.run(),
            Service::Sort(s) => s.run(),
            Service::NotFound => Response::error("Not Found", 404),
        }
    }
}

pub(crate) trait Help {
    fn help() -> Result<Response>;
}
