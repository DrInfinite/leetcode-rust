use crate::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last_occurance = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            last_occurance.insert(c, i);
        }

        let mut result: Vec<i32> = Vec::new();
        let mut start: usize = 0;
        let mut end: usize = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(&last) = last_occurance.get(&c) {
                end = end.max(last);
            }
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }

        return result;
    }
}
