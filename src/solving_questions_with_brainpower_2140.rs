use crate::Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut cache = vec![0i64; questions.len() + 1];
        return questions
            .iter()
            .enumerate()
            .rev()
            .fold(0, |acc, (id, vector)| {
                let [points, brainpower] = vector[..] else {
                    unreachable!("unexpected input format")
                };

                let next = questions.len().min(id + brainpower as usize + 1);
                let take = points as i64 + cache[next];
                cache[id] = acc.max(take);

                return cache[id];
            });
    }
}
