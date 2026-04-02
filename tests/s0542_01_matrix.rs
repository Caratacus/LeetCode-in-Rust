// Tests for Problem 0542: 01 Matrix
// Java reference: src/test/java/g0501_0600/s0542_01_matrix/SolutionTest.java

use leetcode_in_rust::s0542::p01_matrix::Solution;

#[test]
fn test_update_matrix() {
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
    );
}

#[test]
fn test_update_matrix2() {
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]
    );
}
