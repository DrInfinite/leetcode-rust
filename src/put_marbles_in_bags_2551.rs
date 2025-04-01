use crate::Solution;

impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        if k == 1 {
            return 0;
        }

        let mut pair_sums: Vec<i32> = weights
            .windows(2)
            .map(|weight| weight[0] + weight[1])
            .collect();
        pair_sums.sort();

        let min_score: i64 = pair_sums
            .iter()
            .take((k - 1) as usize)
            .map(|&min| min as i64)
            .sum();
        let max_score: i64 = pair_sums
            .iter()
            .rev()
            .take((k - 1) as usize)
            .map(|&max| max as i64)
            .sum();

        return max_score - min_score;
    }
}
