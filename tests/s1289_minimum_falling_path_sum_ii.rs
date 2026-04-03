// Tests for Problem 1289: Minimum Falling Path Sum II
// Java reference: src/test/java/g1201_1300/s1289_minimum_falling_path_sum_ii/SolutionTest.java

use leetcode_in_rust::s1289::minimum_falling_path_sum_ii::Solution;

#[test]
fn test_min_falling_path_sum() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        13
    );
}

#[test]
fn test_min_falling_path_sum2() {
    assert_eq!(Solution::min_falling_path_sum(vec![vec![7]]), 7);
}
