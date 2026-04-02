// Tests for Problem 0931: Minimum Falling Path Sum
// Java reference: src/test/java/g0901_1000/s0931_minimum_falling_path_sum/SolutionTest.java

use leetcode_in_rust::s0931::minimum_falling_path_sum::Solution;

#[test]
fn test_min_falling_path_sum() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]]),
        13
    );
}

#[test]
fn test_min_falling_path_sum2() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![-19, 57], vec![-40, -5]]),
        -59
    );
}
