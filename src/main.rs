#![allow(dead_code)]
#![allow(clippy::needless_return)]

mod binary_tree_node;

mod lowest_common_ancestor_of_deepest_leaves_1123;
mod maximum_number_of_points_from_grid_queries_2503;
mod maximum_value_of_an_ordered_triplet_i_2873;
mod maximum_value_of_an_ordered_triplet_ii_2874;
mod minimum_number_of_operations_to_make_elements_in_array_distinct_3396;
mod partition_equal_subset_sum_416;
mod partition_labels_763;
mod put_marbles_in_bags_2551;
mod solving_questions_with_brainpower_2140;
mod trapping_rain_water_42;

pub struct Solution;

pub struct TreeHelper;

fn main() {
    let result: String = format!(
        "{:?}",
        Solution::maximum_triplet_value([1, 10, 3, 4, 19].to_vec())
    );
    println!("{}", result);
}
