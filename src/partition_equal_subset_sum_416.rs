use crate::Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();

        if sum % 2 == 1 {
            return false;
        }

        let target = (sum / 2) as usize;
        let mut dp = vec![false; target + 1];
        dp[0] = true;

        for &num in nums.iter() {
            for i in (0..=target).rev() {
                if dp[i] == false || i + num as usize > target {
                    continue;
                }

                dp[i + num as usize] = true;
            }
        }

        return dp[target];
    }
}
