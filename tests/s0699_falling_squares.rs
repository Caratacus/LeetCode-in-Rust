// Tests for Problem 0699: Falling Squares
// Java reference: src/test/java/g0601_0700/s0699_falling_squares/SolutionTest.java

use leetcode_in_rust::s0699::falling_squares::Solution;

#[test]
fn test_falling_squares() {
    assert_eq!(
        Solution::falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
        vec![2, 5, 5]
    );
}

#[test]
fn test_falling_squares2() {
    assert_eq!(
        Solution::falling_squares(vec![vec![100, 100], vec![200, 100]]),
        vec![100, 100]
    );
}
