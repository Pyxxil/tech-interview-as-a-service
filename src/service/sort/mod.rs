use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Response, Result};

mod algorithm;
use algorithm::Algorithm;

use crate::service::Service;

pub const NAME: &str = "sort";

// We don't want this to timeout, so provide a soft cap
const CAP: u64 = 20;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    values: Vec<i64>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            values: vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {
    #[serde(default)]
    #[serde(skip_serializing)]
    data: Data,
    #[serde(default)]
    algorithm: Algorithm,
}

impl Default for Sort {
    fn default() -> Self {
        Self {
            data: Data::default(),
            algorithm: Algorithm::Bubble,
        }
    }
}

impl Service for Sort {
    type Body = Data;

    fn help(status: Option<(String, u16)>) -> Response {
        let help = format!(
            "Help: Try appending the following to the url (without the quotes): '?{}'\n\nAnd send a JSON body that looks similar to: {}",
            serde_urlencoded::to_string(&Sort::default()).unwrap(),
            json!(Data::default())
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
    /// Create this service from the query provided in the url. If the query does not exist,
    /// then respond with a help string.
    ///
    /// For Sorting, we also want to be provided the body as taking the array in via url params
    /// wouldn't be that user friendly.
    ///
    fn create(
        body: Option<Result<Self::Body>>,
        query: &str,
    ) -> std::result::Result<Self, Response> {
        serde_urlencoded::from_str::<Sort>(query)
            .map_err(|err| Sort::help(Some((err.to_string(), 400))))
            .and_then(|mut sort| {
                // We can unwrap this without checking because we control
                // whether we're passed this or not
                body.unwrap()
                    .map(|body| {
                        sort.data = body;
                        sort
                    })
                    .map_err(|err| Sort::help(Some((err.to_string(), 400))))
            })
    }

    ///
    /// Run the service.
    ///
    /// For `Sort`, this will not even bother to complete the request if the number of
    /// elements supplied is greater than `CAP` so as to night have Cloudflare workes
    /// timeout (and also to reduce the memory footprint).
    ///
    fn response(self) -> Result<Response> {
        if self.data.values.len() > CAP.try_into().unwrap() {
            Response::error(
                format!("The number of values must be fewer than {}", CAP),
                400,
            )
        } else {
            let (values, steps) = self.algorithm.run(self.data.values);

            Response::from_json(&json!({ "values": values, "steps": steps }))
        }
    }
}
