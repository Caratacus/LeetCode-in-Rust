// Tests for Problem 0867: Transpose Matrix
// Java reference: src/test/java/g0801_0900/s0867_transpose_matrix/SolutionTest.java

use leetcode_in_rust::s0867::transpose_matrix::Solution;

#[test]
fn test_transpose() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
}

#[test]
fn test_transpose2() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![vec![1, 4], vec![2, 5], vec![3, 6]]
    );
}
