use serde::{Deserialize, Serialize};
use worker::{Response, Result};

use crate::services::SERVICES;
use crate::Service;

fn help_message() -> String {
    format!(
        "Try one of the following endpoints:\n{}\n",
        SERVICES
            .iter()
            .filter(|service| !service.is_empty())
            .map(|service| format!("\t{service}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Index;

impl Service for Index {
    const NAME: &'static str = "";

    fn error(_message: &str, _status_code: u16) -> Result<Response> {
        Self::help()
    }

    fn help() -> Result<Response> {
        Response::ok(help_message())
    }

    fn response(self) -> Result<Response> {
        Self::help()
    }
}
