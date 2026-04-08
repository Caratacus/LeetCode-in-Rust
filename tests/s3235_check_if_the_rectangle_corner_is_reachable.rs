// Tests for Problem 3235: Check if the Rectangle Corner Is Reachable
// Java reference: src/test/java/g3201_3300/s3235_check_if_the_rectangle_corner_is_reachable/SolutionTest.java

use leetcode_in_rust::s3235::check_if_the_rectangle_corner_is_reachable::Solution;

#[test]
fn test_can_reach_corner() {
    assert_eq!(Solution::can_reach_corner(3, 4, vec![vec![2, 1, 1]]), true);
}

#[test]
fn test_can_reach_corner2() {
    assert_eq!(Solution::can_reach_corner(3, 3, vec![vec![1, 1, 2]]), false);
}

#[test]
fn test_can_reach_corner3() {
    assert_eq!(
        Solution::can_reach_corner(3, 3, vec![vec![2, 1, 1], vec![1, 2, 1]]),
        false
    );
}

#[test]
fn test_can_reach_corner4() {
    assert_eq!(Solution::can_reach_corner(4, 4, vec![vec![5, 5, 1]]), true);
}
