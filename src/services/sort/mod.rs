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

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Sort {
    #[serde(default)]
    #[serde(skip_serializing)]
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
    fn help(status: Option<(String, u16)>) -> Response {
        let help = format!(
            "Help: Try sending a JSON body with the following: {}",
            json!(Self::default())
        );

        if let Some((err, status)) = status {
            if status >= 400 {
                Response::error(format!("{}\n\n{}", err, help), status).unwrap()
            } else {
                Response::ok(format!("{}\n\n{}", err, help)).unwrap()
            }
        } else {
            Response::ok(help).unwrap()
        }
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
