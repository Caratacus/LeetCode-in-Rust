// Tests for Problem 1074: Number of Submatrices That Sum to Target
// Java reference: src/test/java/g1001_1100/s1074_number_of_submatrices_that_sum_to_target/SolutionTest.java

use leetcode_in_rust::s1074::number_of_submatrices_that_sum_to_target::Solution;

#[test]
fn test_num_submatrix_sum_target() {
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0),
        4
    );
}

#[test]
fn test_num_submatrix_sum_target2() {
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
        5
    );
}
