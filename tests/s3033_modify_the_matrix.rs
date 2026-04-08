// Tests for Problem 3033: Modify the Matrix
// Java reference: src/test/java/g3001_3100/s3033_modify_the_matrix/SolutionTest.java

use leetcode_in_rust::s3033::modify_the_matrix::Solution;

#[test]
fn test_modified_matrix() {
    let matrix = vec![vec![1, 2, -1], vec![4, -1, 6], vec![7, 8, 9]];
    let expected = vec![vec![1, 2, 9], vec![4, 8, 6], vec![7, 8, 9]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
}

#[test]
fn test_modified_matrix2() {
    let matrix = vec![vec![3, -1], vec![5, 2]];
    let expected = vec![vec![3, 2], vec![5, 2]];
    assert_eq!(Solution::modified_matrix(matrix), expected);
}
