use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

use crate::utils::min_max;

// We don't want this to timeout, so provide a soft cap
const CAP: u64 = 1000;

fn default_fizz() -> String {
    String::from("Fizz")
}

fn default_buzz() -> String {
    String::from("Buzz")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FizzBuzz {
    #[serde(default)]
    from: u64,
    to: u64,
    #[serde(default)]
    inclusive: bool,
    #[serde(default = "default_fizz")]
    fizz: String,
    #[serde(default = "default_buzz")]
    buzz: String,
}

impl Default for FizzBuzz {
    fn default() -> Self {
        Self {
            from: 0,
            to: 100,
            inclusive: false,
            fizz: default_fizz(),
            buzz: default_buzz(),
        }
    }
}

impl FizzBuzz {
    ///
    /// Run the service.
    ///
    /// For FizzBuzz, this will not even bother to complete the request if the difference
    /// between the two is higher than the `CAP` so as to not timeout the request (and also
    /// to reduce the memory footprint).
    ///
    /// **TODO**: Have a cap on the length of the two strings that are generated
    ///
    pub fn run(self) -> Result<Response> {
        let (from, to) = min_max(self.from, self.to);

        if to - from > CAP {
            return Response::error(
                "The difference between 'to' and 'from' must be no greater than 1000",
                400,
            );
        }

        let values = (from..(to + if self.inclusive { 1 } else { 0 }))
            .into_iter()
            .map(|a| {
                if a % 15 == 0 {
                    format!("{}{}", self.fizz, self.buzz)
                } else if a % 5 == 0 {
                    self.buzz.clone()
                } else if a % 3 == 0 {
                    self.fizz.clone()
                } else {
                    a.to_string()
                }
            })
            .collect::<Vec<String>>();

        Response::from_json(&json!({ "values": values }))
    }

    ///
    /// Create this service based on the query provided in the url. If the query does not exist,
    /// then respond with a help string.
    ///
    pub fn create(query: Option<&str>) -> std::result::Result<FizzBuzz, Response> {
        if let Some(query) = query {
            let fizz = serde_urlencoded::from_str(query)
                .map_err(|err| FizzBuzz::help(Some((err.to_string(), 400))))?;

            Ok(fizz)
        } else {
            Err(FizzBuzz::help(None))
        }
    }

    fn help(status: Option<(String, u16)>) -> Response {
        let help = format!(
            "Try appending the following to the url (without the quotes): '?{}'",
            serde_urlencoded::to_string(&FizzBuzz::default()).unwrap()
        );

        if let Some((err, status)) = status {
            Response::error(format!("{}\n\n{}", err, help), status).unwrap()
        } else {
            Response::ok(help).unwrap()
        }
    }
}
