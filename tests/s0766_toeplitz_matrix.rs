// Tests for Problem 0766: Toeplitz Matrix
// Java reference: src/test/java/g0701_0800/s0766_toeplitz_matrix/SolutionTest.java

use leetcode_in_rust::s0766::toeplitz_matrix::Solution;

#[test]
fn test_is_toeplitz_matrix() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4],
            vec![5, 1, 2, 3],
            vec![9, 5, 1, 2]
        ]),
        true
    );
}

#[test]
fn test_is_toeplitz_matrix2() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![vec![1, 2], vec![2, 2]]),
        false
    );
}

#[test]
fn test_is_toeplitz_matrix3() {
    assert_eq!(
        Solution::is_toeplitz_matrix(vec![
            vec![1, 2, 3, 4, 5, 9],
            vec![4, 1, 2, 3, 4, 5],
            vec![7, 4, 1, 2, 3, 4],
            vec![2, 7, 4, 1, 2, 3],
            vec![5, 2, 7, 4, 1, 2]
        ]),
        true
    );
}
