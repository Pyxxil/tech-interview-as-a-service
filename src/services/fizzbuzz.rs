use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Response, Result};

use crate::traits::Help;
use crate::utils::min_max;
use crate::Service;

pub const NAME: &str = "fizzbuzz";

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

impl Service for FizzBuzz {
    fn error(message: &str, status_code: u16) -> Result<Response> {
        Response::error(
            format!("{}\n\n{}", message, FizzBuzz::help_message()),
            status_code,
        )
    }

    fn help() -> Result<Response> {
        Response::ok(FizzBuzz::help_message())
    }

    ///
    /// Run the service.
    ///
    /// For `FizzBuzz`, this will not even bother to complete the request if the difference
    /// between the two is higher than the `CAP` so as to not timeout the request (and also
    /// to reduce the memory footprint).
    ///
    /// **TODO**: Have a cap on the length of the two strings that are generated
    ///
    fn response(self) -> Result<Response> {
        let (from, to) = min_max(self.from, self.to);

        if to - from > CAP {
            Response::error(
                format!(
                    "The difference between 'to' and 'from' must be no greater than {}",
                    CAP
                ),
                400,
            )
        } else {
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
    }
}
