use crate::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let scan_callback = |m: &mut i32, x: &i32| {
            *m = std::cmp::max(*m, *x);
            return Some(*m);
        };

        let area_left: i32 = height.iter().scan(0, scan_callback).sum();
        let area_right: i32 = height.iter().rev().scan(0, scan_callback).sum();

        let area_landscape: i32 = height.iter().sum();
        let area_box = *height.iter().max().unwrap() * height.len() as i32;

        return (area_left + area_right) - (area_box + area_landscape);
    }
}
