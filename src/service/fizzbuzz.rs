use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

use crate::utils::min_max;

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
    pub fn run(self) -> Result<Response> {
        let (from, to) = min_max(self.from, self.to);

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

    pub fn from_params(query: Option<&str>) -> std::result::Result<FizzBuzz, Response> {
        if let Some(query) = query {
            let fizz = serde_urlencoded::from_str(query)
                .map_err(|err| Response::error(err.to_string(), 400).unwrap())?;

            Ok(fizz)
        } else {
            Err(Response::ok(format!(
                "Try appending the following to the url (without the quotes): '?{}'",
                serde_urlencoded::to_string(&FizzBuzz::default()).unwrap()
            ))
            .unwrap())
        }
    }
}
