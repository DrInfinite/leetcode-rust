use crate::Solution;

impl Solution {
    pub fn maximum_triplet_value_1(nums: Vec<i32>) -> i64 {
        let length = nums.len();
        let nums_64: Vec<i64> = nums.iter().map(|&num| num as i64).collect();

        let mut prefix_max = vec![0; length];
        prefix_max[0] = nums_64[0];
        for i in 1..length {
            prefix_max[i] = prefix_max[i - 1].max(nums_64[i]);
        }

        let mut result = 0;

        for j in 1..length - 1 {
            let max_ij = prefix_max[j - 1] - nums_64[j];
            let max_k = *nums_64[j + 1..].iter().max().unwrap();
            result = result.max(max_ij * max_k);
        }

        return result;
    }
}
