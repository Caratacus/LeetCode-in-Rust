// Tests for Problem 1770: Maximum Score from Performing Multiplication Operations
// Java reference: src/test/java/g1701_1800/s1770_maximum_score_from_performing_multiplication_operations/SolutionTest.java

use leetcode_in_rust::s1770::maximum_score_from_performing_multiplication_operations::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]), 14);
}

#[test]
fn test_maximum_score2() {
    assert_eq!(
        Solution::maximum_score(vec![-5, -3, -3, -2, 7, 1], vec![-10, -5, 3, 4, 6]),
        102
    );
}
