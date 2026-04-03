// Tests for Problem 1637: Widest Vertical Area Between Two Points Containing No Points
// Java reference: src/test/java/g1601_1700/s1637_widest_vertical_area_between_two_points_containing_no_points/SolutionTest.java

use leetcode_in_rust::s1637::widest_vertical_area_between_two_points_containing_no_points::Solution;

#[test]
fn test_max_width_of_vertical_area() {
    assert_eq!(Solution::max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]), 1);
}

#[test]
fn test_max_width_of_vertical_area2() {
    assert_eq!(
        Solution::max_width_of_vertical_area(vec![vec![3, 1], vec![9, 0], vec![1, 0], vec![1, 4], vec![5, 3], vec![8, 8]]),
        3
    );
}
