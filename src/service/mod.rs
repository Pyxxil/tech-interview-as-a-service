use worker::{Response, Result};

use crate::service::fizzbuzz::FizzBuzz;

mod fizzbuzz;

#[derive(Debug)]
pub enum Service {
    FizzBuzz(FizzBuzz),
    NotFound,
}

impl Service {
    pub fn from(url: Result<url::Url>) -> std::result::Result<Service, Response> {
        if_chain! {
            if let Ok(url) = url;
            if let Some(mut path) = url.path_segments();
            if let Some(service) = path.next();

            then {
                return match service.to_ascii_lowercase().as_str() {
                    "fizzbuzz" => FizzBuzz::from(url.query()).map(Service::FizzBuzz),
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
            Service::NotFound => Response::error("Not Found", 404),
        }
    }
}

pub(crate) trait Help {
    fn help() -> Result<Response>;
}
