// Tests for Problem 3643: Flip Square Submatrix Vertically
// Java reference: src/test/java/g3601_3700/s3643_flip_square_submatrix_vertically/SolutionTest.java
use leetcode_in_rust::s3643::flip_square_submatrix_vertically::Solution;
#[test]
fn test_reverse_submatrix() {
    assert_eq!(
        Solution::reverse_submatrix(vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12], vec![13, 14, 15, 16]], 1, 0, 3),
        vec![vec![1, 2, 3, 4], vec![13, 14, 15, 8], vec![9, 10, 11, 12], vec![5, 6, 7, 16]]
    );
}
#[test]
fn test_reverse_submatrix2() {
    assert_eq!(
        Solution::reverse_submatrix(vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]], 0, 2, 2),
        vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]]
    );
}
