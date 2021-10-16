use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Response, Result};

mod algorithm;
use algorithm::SortingAlgorithm;

use crate::traits::{Algorithm, Service};

pub const NAME: &str = "sort";

// We don't want this to timeout, so provide a soft cap
const CAP: u64 = 20;

fn help_message() -> String {
    format!(
        "Help: Try sending a JSON body with the following:\n{}\n",
        serde_json::to_string_pretty(&Sort::default()).unwrap()
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Sort {
    values: Vec<i64>,
    #[serde(default)]
    algorithm: SortingAlgorithm,
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            values: vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
            algorithm: SortingAlgorithm::Bubble,
        }
    }
}

impl Service for Sort {
    fn error(message: &str, status_code: u16) -> Result<Response> {
        Response::error(format!("{}\n\n{}", message, help_message()), status_code)
    }

    fn help() -> Result<Response> {
        Response::ok(help_message())
    }

    ///
    /// Run the service.
    ///
    /// For `Sort`, this will not even bother to complete the request if the number of
    /// elements supplied is greater than `CAP` so as to night have Cloudflare workes
    /// timeout (and also to reduce the memory footprint).
    ///
    fn response(self) -> Result<Response> {
        if self.values.len() > CAP.try_into().unwrap() {
            Response::error(
                format!("The number of values must be fewer than {}", CAP),
                400,
            )
        } else {
            let (values, steps) = self.algorithm.run(self.values);

            Response::from_json(&json!({ "values": values, "steps": steps }))
        }
    }
}
