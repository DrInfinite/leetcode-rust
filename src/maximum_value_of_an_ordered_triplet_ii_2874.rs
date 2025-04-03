use crate::Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut suffix = nums.clone();
        for i in (0..suffix.len()).rev().skip(1) {
            suffix[i] = suffix[i].max(suffix[i + 1]);
        }

        let mut maximum = 0;
        let mut prefix = nums[0];
        for i in 1..nums.len() - 1 {
            maximum = maximum.max((prefix - nums[i]) as i64 * suffix[i + 1] as i64);
            prefix = prefix.max(nums[i]);
        }

        return maximum;
    }
}
