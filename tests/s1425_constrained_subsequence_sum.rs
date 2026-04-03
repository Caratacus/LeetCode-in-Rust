// Tests for Problem 1425: Constrained Subsequence Sum
// Java reference: src/test/java/g1401_1500/s1425_constrained_subsequence_sum/SolutionTest.java

use leetcode_in_rust::s1425::constrained_subsequence_sum::Solution;

#[test]
fn test_constrained_subset_sum() {
    assert_eq!(Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2), 37);
}

#[test]
fn test_constrained_subset_sum2() {
    assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);
}

#[test]
fn test_constrained_subset_sum3() {
    assert_eq!(Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2), 23);
}
