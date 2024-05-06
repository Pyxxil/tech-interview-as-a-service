use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Response, Result};

use crate::traits::Help;
use crate::utils::min_max;
use crate::Service;

// We don't want this to timeout, so provide a soft cap
const CAP: u64 = 1000;

fn default_fizz() -> String {
    String::from("Fizz")
}

fn default_buzz() -> String {
    String::from("Buzz")
}

fn default_fizzer() -> u64 {
    3
}

fn default_buzzer() -> u64 {
    5
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
    #[serde(default = "default_fizzer")]
    fizzer: u64,
    #[serde(default = "default_buzzer")]
    buzzer: u64,
}

impl Default for FizzBuzz {
    fn default() -> Self {
        Self {
            from: 0,
            to: 100,
            inclusive: false,
            fizz: default_fizz(),
            buzz: default_buzz(),
            fizzer: default_fizzer(),
            buzzer: default_buzzer(),
        }
    }
}

impl Service for FizzBuzz {
    const NAME: &'static str = "fizzbuzz";

    fn error(message: &str, status_code: u16) -> Result<Response> {
        Response::error(
            format!("{message}\n\n{}", Self::help_message()),
            status_code,
        )
    }

    fn help() -> Result<Response> {
        Response::ok(Self::help_message())
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
                format!("The difference between 'to' and 'from' must be no greater than {CAP}"),
                400,
            )
        } else {
            let fizzbuzz = format!("{}{}", self.fizz, self.buzz);
            let values = (from..(to + u64::from(self.inclusive)))
                .map(|a| {
                    if a % self.fizzer == 0 && a % self.buzzer == 0 {
                        fizzbuzz.clone()
                    } else if a % self.buzzer == 0 {
                        self.buzz.clone()
                    } else if a % self.fizzer == 0 {
                        self.fizz.clone()
                    } else {
                        a.to_string()
                    }
                })
                .collect::<Vec<_>>();

            Response::from_json(&json!({ "values": values }))
        }
    }
}
