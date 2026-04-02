// Tests for Problem 0566: Reshape the Matrix
// Java reference: src/test/java/g0501_0600/s0566_reshape_the_matrix/SolutionTest.java

use leetcode_in_rust::s0566::reshape_the_matrix::Solution;

#[test]
fn test_matrix_reshape() {
    let mat = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::matrix_reshape(mat, 1, 4), vec![vec![1, 2, 3, 4]]);
}

#[test]
fn test_matrix_reshape2() {
    let mat = vec![vec![1, 2], vec![3, 4]];
    let result = Solution::matrix_reshape(mat.clone(), 2, 4);
    assert_eq!(result, mat);
}
