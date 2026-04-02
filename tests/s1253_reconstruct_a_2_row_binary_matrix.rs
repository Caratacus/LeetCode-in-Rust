// Tests for Problem 1253: Reconstruct a 2-Row Binary Matrix
// Java reference: src/test/java/g1201_1300/s1253_reconstruct_a_2_row_binary_matrix/SolutionTest.java

use leetcode_in_rust::s1253::reconstruct_a_2_row_binary_matrix::Solution;

#[test]
fn test_reconstruct_matrix() {
    assert_eq!(
        Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]),
        vec![vec![0, 1, 1], vec![1, 0, 0]]
    );
}

#[test]
fn test_reconstruct_matrix2() {
    let result: Vec<Vec<i32>> = Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]);
    assert!(result.is_empty());
}

#[test]
fn test_reconstruct_matrix3() {
    assert_eq!(
        Solution::reconstruct_matrix(5, 5, vec![2, 1, 2, 0, 1, 0, 1, 2, 0, 1]),
        vec![vec![1, 0, 1, 0, 0, 0, 1, 1, 0, 1], vec![1, 1, 1, 0, 1, 0, 0, 1, 0, 0]]
    );
}
