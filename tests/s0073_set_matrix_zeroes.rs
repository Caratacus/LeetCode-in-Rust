// Tests for Problem 0073: Set Matrix Zeroes
// Java reference: src/test/java/g0001_0100/s0073_set_matrix_zeroes/SolutionTest.java

use leetcode_in_rust::s0073::set_matrix_zeroes::Solution;

#[test]
fn test_set_zeroes() {
    let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
}

#[test]
fn test_set_zeroes2() {
    let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(&mut matrix);
    assert_eq!(matrix, vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]);
}
