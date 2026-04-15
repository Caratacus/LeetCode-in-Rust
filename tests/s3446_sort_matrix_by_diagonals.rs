// Tests for Problem 3446: Sort Matrix by Diagonals
// Java reference: src/test/java/g3401_3500/s3446_sort_matrix_by_diagonals/SolutionTest.java

use leetcode_in_rust::s3446::sort_matrix_by_diagonals::Solution;

#[test]
fn test_sort_matrix() {
    assert_eq!(
        Solution::sort_matrix(vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]]),
        vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]
    );
}

#[test]
fn test_sort_matrix2() {
    assert_eq!(
        Solution::sort_matrix(vec![vec![0, 1], vec![1, 2]]),
        vec![vec![2, 1], vec![1, 0]]
    );
}

#[test]
fn test_sort_matrix3() {
    assert_eq!(Solution::sort_matrix(vec![vec![1]]), vec![vec![1]]);
}
