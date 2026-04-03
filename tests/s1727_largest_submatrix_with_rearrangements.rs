// Tests for Problem 1727: Largest Submatrix With Rearrangements
// Java reference: src/test/java/g1701_1800/s1727_largest_submatrix_with_rearrangements/SolutionTest.java

use leetcode_in_rust::s1727::largest_submatrix_with_rearrangements::Solution;

#[test]
fn test_largest_submatrix() {
    assert_eq!(
        Solution::largest_submatrix(vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]),
        4
    );
}

#[test]
fn test_largest_submatrix2() {
    assert_eq!(
        Solution::largest_submatrix(vec![vec![1, 0, 1, 0, 1]]),
        3
    );
}

#[test]
fn test_largest_submatrix3() {
    assert_eq!(
        Solution::largest_submatrix(vec![vec![1, 1, 0], vec![1, 0, 1]]),
        2
    );
}
