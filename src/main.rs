#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod maximum_number_of_points_from_grid_queries_2503;
mod partition_labels_763;
mod put_marbles_in_bags_2551;
mod solving_questions_with_brainpower_2140;
mod trapping_rain_water_42;

pub struct Solution;

fn main() {
    let result: String = format!(
        "{:?}",
        Solution::trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1].to_vec())
    );
    println!("Result: {}", result);
}
