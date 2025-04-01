use std::collections::BinaryHeap;

use crate::Solution;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut g = grid;

        let (mut q, mut result, mut count) = (
            BinaryHeap::from_iter([(-g[0][0], 0, 0)]),
            vec![0; queries.len()],
            0,
        );

        let mut idx: Vec<_> = (0..queries.len()).collect();
        idx.sort_unstable_by_key(|&x| queries[x]);

        for i in idx {
            while let Some(&(v, y, x)) = q.peek() {
                if -v >= queries[i] {
                    break;
                };

                q.pop();

                if g[y][x] < 0 {
                    continue;
                };

                g[y][x] = -1;
                count += 1;

                for (i, j) in [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)] {
                    if i < g.len() && j < g[0].len() {
                        q.push((-g[i][j], i, j))
                    }
                }
            }
            result[i] = count;
        }

        return result;
    }
}
