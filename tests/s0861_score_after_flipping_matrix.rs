// Tests for Problem 0861: Score After Flipping Matrix
// Java reference: src/test/java/g0801_0900/s0861_score_after_flipping_matrix/SolutionTest.java

use leetcode_in_rust::s0861::score_after_flipping_matrix::Solution;

#[test]
fn test_matrix_score() {
    assert_eq!(
        Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]]),
        39
    );
}

#[test]
fn test_matrix_score2() {
    assert_eq!(Solution::matrix_score(vec![vec![0]]), 1);
}
