// Tests for Problem 0064: Minimum Path Sum
// Java reference: src/test/java/g0001_0100/s0064_minimum_path_sum/SolutionTest.java

use leetcode_in_rust::s0064::minimum_path_sum::Solution;

#[test]
fn test_min_path_sum() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(Solution::min_path_sum(grid), 7);
}

#[test]
fn test_min_path_sum2() {
    let grid = vec![vec![1, 2, 3], vec![4, 5, 6]];
    assert_eq!(Solution::min_path_sum(grid), 12);
}
