use crate::Solution;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        return (0..nums.len())
            .rev()
            .scan(HashSet::new(), |set, index| {
                set.insert(nums[index]).then_some(index)
            })
            .last()
            .map_or(0, |index| ((index + 2) / 3) as i32);
    }
}
