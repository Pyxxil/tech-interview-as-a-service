use worker::{Request, Response, Result};

mod fizzbuzz;
use crate::service::fizzbuzz::FizzBuzz;
mod sort;
use crate::service::sort::Sort;

#[derive(Debug)]
pub enum Service {
    FizzBuzz(FizzBuzz),
    Sort(Sort),
    Help(Response),
    NotFound,
}

pub enum Type {
    Help,
    Req,
}

impl Service {
    async fn _from(t: Type, mut context: Request) -> std::result::Result<Service, Response> {
        if_chain! {
            if let Ok(url) = context.url();
            if let Some(mut path) = url.path_segments();
            if let Some(service) = path.next();

            then {
                return match (t, service.to_ascii_lowercase().as_str()) {
                    (Type::Req, fizzbuzz::NAME) => {
                        FizzBuzz::from(url.query()).map(Service::FizzBuzz)
                    },
                    (Type::Help, fizzbuzz::NAME) =>
                        Ok(Service::Help(FizzBuzz::help(Some((
                            "Try sending a POST request to this endpoint".to_string(),
                            200,
                        ))))),

                    (Type::Req, sort::NAME) => Sort::from(context.json().await, url.query()).map
                    (Service::Sort),
                    (Type::Help, sort::NAME) => Ok(Service::Help(Sort::help(Some((
                        "Try sending a POST request to this endpoint".to_string(),
                        200,
                    ))))),

                    _ => Ok(Service::NotFound)
                };
            }
        }

        Ok(Service::NotFound)
    }

    pub async fn from(context: Request) -> std::result::Result<Service, Response> {
        Service::_from(Type::Req, context).await
    }

    pub async fn help(context: Request) -> std::result::Result<Service, Response> {
        Service::_from(Type::Help, context).await
    }

    pub fn response(self) -> Result<Response> {
        match self {
            Service::FizzBuzz(s) => s.run(),
            Service::Sort(s) => s.run(),
            Service::Help(res) => Ok(res),
            Service::NotFound => Response::error("Not Found", 404),
        }
    }
}
