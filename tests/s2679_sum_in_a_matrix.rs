// Tests for Problem 2679: Sum in a Matrix
// Java reference: src/test/java/g2601_2700/s2679_sum_in_a_matrix/SolutionTest.java

use leetcode_in_rust::s2679::sum_in_a_matrix::Solution;

#[test]
fn test_matrix_sum() {
    assert_eq!(
        Solution::matrix_sum(vec![
            vec![7, 2, 1],
            vec![6, 4, 2],
            vec![6, 5, 3],
            vec![3, 2, 1]
        ]),
        15
    );
}

#[test]
fn test_matrix_sum2() {
    assert_eq!(Solution::matrix_sum(vec![vec![1]]), 1);
}
