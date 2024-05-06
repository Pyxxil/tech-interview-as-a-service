use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::{Response, Result};

use crate::traits::{Algorithm, Help, Service};

// We don't want this to timeout, so provide a soft cap
const CAP: usize = 20;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortingAlgorithm {
    Bubble,
    Merge,
    Insertion,
    Selection,
}

impl Default for SortingAlgorithm {
    fn default() -> Self {
        Self::Bubble
    }
}

pub enum StepAction<'a, T: ?Sized> {
    SwapAction(usize, usize, &'a T),
    Split(&'a T, &'a T),
    Merge(&'a T),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Sort {
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

impl Sort {
    ///
    /// Perform the merging of the two lists for Merge Sort.
    ///
    fn merge(&self, l: &[i64], r: &[i64], steps: &mut Vec<serde_json::Value>) -> Vec<i64> {
        let mut merged = Vec::new();

        let (mut left_idx, mut right_idx) = (0, 0);

        while left_idx < l.len() && right_idx < r.len() {
            if l[left_idx] <= r[right_idx] {
                merged.push(l[left_idx]);
                left_idx += 1;
            } else {
                merged.push(r[right_idx]);
                right_idx += 1;
            }

            steps.push(self.step(StepAction::Merge(&merged)));
        }

        while left_idx < l.len() {
            merged.push(l[left_idx]);
            left_idx += 1;
            steps.push(self.step(StepAction::Merge(&merged)));
        }

        while right_idx < r.len() {
            merged.push(r[right_idx]);
            right_idx += 1;
            steps.push(self.step(StepAction::Merge(&merged)));
        }

        merged
    }

    fn merge_sort(&self, m: &[i64], steps: &mut Vec<serde_json::Value>) -> Vec<i64> {
        if m.len() <= 1 {
            return m.to_vec();
        }

        let (left, right) = m.split_at(m.len() / 2);

        steps.push(self.step(StepAction::Split(left, right)));

        self.merge(
            &self.merge_sort(left, steps),
            &self.merge_sort(right, steps),
            steps,
        )
    }
}

impl Service for Sort {
    const NAME: &'static str = "sort";

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
    /// For `Sort`, this will not even bother to complete the request if the number of
    /// elements supplied is greater than `CAP` so as to not have the Cloudflare worker
    /// timeout (and also to reduce the memory footprint).
    ///
    fn response(self) -> Result<Response> {
        if self.values.len() > CAP {
            Response::error(
                format!("The number of values must be fewer than {CAP}"),
                400,
            )
        } else {
            let (values, steps) = self.run(());

            Response::from_json(&json!({ "values": values, "steps": steps }))
        }
    }
}

impl Algorithm for Sort {
    type Input = ();
    type Output = (Vec<i64>, Vec<serde_json::Value>);
    type Step = serde_json::Value;
    type Action<'a> = StepAction<'a, [i64]>;

    fn run(mut self, _: Self::Input) -> Self::Output {
        if self.values.len() <= 1 {
            return (self.values, Vec::new());
        }

        let mut steps = Vec::with_capacity(self.values.len() / 2);

        match self.algorithm {
            SortingAlgorithm::Bubble => {
                let mut length = self.values.len();

                loop {
                    let mut final_index = 0;

                    for i in 1..length {
                        if self.values[i - 1] > self.values[i] {
                            self.values.swap(i - 1, i);

                            steps.push(self.step(StepAction::SwapAction(i - 1, i, &self.values)));
                            final_index = i;
                        }
                    }

                    length = final_index;

                    if length <= 1 {
                        break;
                    }
                }

                (self.values.clone(), steps)
            }

            SortingAlgorithm::Merge => (self.merge_sort(&self.values, &mut steps), steps),

            SortingAlgorithm::Insertion => {
                for i in 0..self.values.len() {
                    let mut j = i;

                    while j > 0 && self.values[j - 1] > self.values[j] {
                        self.values.swap(j, j - 1);
                        steps.push(self.step(StepAction::SwapAction(j, j - 1, &self.values)));
                        j -= 1;
                    }
                }

                (self.values.clone(), steps)
            }

            SortingAlgorithm::Selection => {
                for left in 0..self.values.len() {
                    let mut smallest = left;

                    for right in left + 1..self.values.len() {
                        if self.values[right] < self.values[left] {
                            smallest = right;
                        }
                    }

                    if left != smallest {
                        self.values.swap(left, smallest);
                        steps.push(self.step(StepAction::SwapAction(left, smallest, &self.values)));
                    }
                }

                (self.values.clone(), steps)
            }
        }
    }

    fn step(&self, step: Self::Action<'_>) -> Self::Step {
        match step {
            Self::Action::<'_>::SwapAction(from, to, list) => {
                json!({
                    "explanation": format!("Swap {} (index: {}) with {} (index: {})", list[to], to, list[from], from),
                    "result": list
                })
            }

            Self::Action::<'_>::Split(left, right) => {
                json!({
                    "explanation": "Split the list in two",
                    "left": left,
                    "right": right
                })
            }

            Self::Action::<'_>::Merge(merged) => {
                json!({
                    "explanation": format!("Merge {} into the result", merged.last().unwrap()),
                    "result": merged
                })
            }
        }
    }
}
