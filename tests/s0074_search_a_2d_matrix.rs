// Tests for Problem 0074: Search a 2D Matrix
// Java reference: src/test/java/g0001_0100/s0074_search_a_2d_matrix/SolutionTest.java

use leetcode_in_rust::s0074::search_a_2d_matrix::Solution;

#[test]
fn test_search_matrix() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(Solution::search_matrix(matrix, 3), true);
}

#[test]
fn test_search_matrix2() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(Solution::search_matrix(matrix, 13), false);
}
