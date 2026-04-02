// Tests for Problem 0059: Spiral Matrix II
// Java reference: src/test/java/g0001_0100/s0059_spiral_matrix_ii/SolutionTest.java

use leetcode_in_rust::s0059::spiral_matrix_ii::Solution;

#[test]
fn test_generate_matrix() {
    let result = Solution::generate_matrix(3);
    assert_eq!(result, vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]);
}

#[test]
fn test_generate_matrix2() {
    let result = Solution::generate_matrix(1);
    assert_eq!(result, vec![vec![1]]);
}
