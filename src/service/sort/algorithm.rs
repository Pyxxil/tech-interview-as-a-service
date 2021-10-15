use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Algorithm {
    Bubble,
    Merge,
}

impl Default for Algorithm {
    fn default() -> Self {
        Self::Bubble
    }
}

impl Algorithm {
    pub fn run(&self, mut list: Vec<i64>) -> (Vec<i64>, Vec<Vec<i64>>) {
        if list.len() <= 1 {
            return (list, Vec::new());
        }

        match self {
            Algorithm::Bubble => {
                let mut steps = Vec::new();

                let mut length = list.len();

                loop {
                    let mut final_index = 0;

                    (1..length).for_each(|idx| {
                        if list[idx - 1] > list[idx] {
                            list.swap(idx - 1, idx);

                            steps.push(list.clone());
                            final_index = idx;
                        }
                    });

                    length = final_index;

                    if length <= 1 {
                        break;
                    }
                }

                (list, steps)
            }

            Algorithm::Merge => {
                fn merge<'a>(l: &'a [i64], r: &'a [i64], steps: &mut Vec<Vec<i64>>) -> Vec<i64> {
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

                        steps.push(merged.clone());
                    }

                    while left_idx < l.len() {
                        merged.push(l[left_idx]);
                        left_idx += 1;
                        steps.push(merged.clone());
                    }

                    while right_idx < r.len() {
                        merged.push(r[right_idx]);
                        right_idx += 1;
                        steps.push(merged.clone());
                    }

                    merged
                }

                fn merge_sort(m: &[i64], steps: &mut Vec<Vec<i64>>) -> Vec<i64> {
                    if m.len() <= 1 {
                        return m.to_vec();
                    }

                    let (left, right) = m.split_at(m.len() / 2);

                    steps.push(left.to_vec());
                    steps.push(right.to_vec());

                    merge(&merge_sort(left, steps), &merge_sort(right, steps), steps)
                }

                let mut steps = Vec::new();
                (merge_sort(&list, &mut steps), steps)
            }
        }
    }
}
