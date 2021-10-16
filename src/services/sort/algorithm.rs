use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::traits::Algorithm;

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

impl Algorithm for SortingAlgorithm {
    type Input = Vec<i64>;
    type Output = (Vec<i64>, Vec<serde_json::Value>);

    fn run(&self, mut list: Self::Input) -> Self::Output {
        if list.len() <= 1 {
            return (list, Vec::new());
        }

        match self {
            SortingAlgorithm::Bubble => {
                let mut steps = Vec::new();
                let mut length = list.len();

                loop {
                    let mut final_index = 0;

                    for i in 1..length {
                        if list[i - 1] > list[i] {
                            list.swap(i - 1, i);

                            steps.push(json!(list.clone()));
                            final_index = i;
                        }
                    }

                    length = final_index;

                    if length <= 1 {
                        break;
                    }
                }

                (list, steps)
            }

            SortingAlgorithm::Merge => {
                let mut steps = Vec::new();
                (merge_sort(&list, &mut steps), steps)
            }

            SortingAlgorithm::Insertion => {
                let mut steps = Vec::new();

                for i in 0..list.len() {
                    let mut j = i;

                    while j > 0 && list[j - 1] > list[j] {
                        list.swap(j, j - 1);
                        steps.push(json!(list));
                        j -= 1;
                    }
                }

                (list, steps)
            }

            SortingAlgorithm::Selection => {
                let mut steps = Vec::new();

                for left in 0..list.len() {
                    let mut smallest = left;

                    for right in left + 1..list.len() {
                        if list[right] < list[left] {
                            smallest = right;
                        }
                    }

                    if left != smallest {
                        list.swap(left, smallest);
                        steps.push(json!(list));
                    }
                }

                (list, steps)
            }
        }
    }

    fn step(&self) {
        todo!()
    }
}

///
/// Perform the merging of the two lists for Merge Sort.
///
fn merge<'a>(l: &'a [i64], r: &'a [i64], steps: &mut Vec<serde_json::Value>) -> Vec<i64> {
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

        steps.push(json!(merged));
    }

    while left_idx < l.len() {
        merged.push(l[left_idx]);
        left_idx += 1;
        steps.push(json!(merged));
    }

    while right_idx < r.len() {
        merged.push(r[right_idx]);
        right_idx += 1;
        steps.push(json!(merged));
    }

    merged
}

fn merge_sort(m: &[i64], steps: &mut Vec<serde_json::Value>) -> Vec<i64> {
    if m.len() <= 1 {
        return m.to_vec();
    }

    let (left, right) = m.split_at(m.len() / 2);

    steps.push(json!(left));
    steps.push(json!(right));

    merge(&merge_sort(left, steps), &merge_sort(right, steps), steps)
}
