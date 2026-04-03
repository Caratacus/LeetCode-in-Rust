// Tests for Problem 1572: Matrix Diagonal Sum
// Java reference: src/test/java/g1501_1600/s1572_matrix_diagonal_sum/SolutionTest.java

use leetcode_in_rust::s1572::matrix_diagonal_sum::Solution;

#[test]
fn test_diagonal_sum() {
    assert_eq!(
        Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        25
    );
}

#[test]
fn test_diagonal_sum2() {
    assert_eq!(
        Solution::diagonal_sum(vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ]),
        8
    );
}

#[test]
fn test_diagonal_sum3() {
    assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
}
