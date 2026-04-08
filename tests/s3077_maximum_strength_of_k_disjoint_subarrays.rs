// Tests for Problem 3077: Maximum Strength of K Disjoint Subarrays
// Java reference: src/test/java/g3001_3100/s3077_maximum_strength_of_k_disjoint_subarrays/SolutionTest.java

use leetcode_in_rust::s3077::maximum_strength_of_k_disjoint_subarrays::Solution;

#[test]
fn test_maximum_strength() {
    assert_eq!(Solution::maximum_strength(vec![1, 2, 3, -1, 2], 3), 22);
}

#[test]
fn test_maximum_strength2() {
    assert_eq!(Solution::maximum_strength(vec![12, -2, -2, -2, -2], 5), 64);
}

#[test]
fn test_maximum_strength3() {
    assert_eq!(Solution::maximum_strength(vec![-1, -2, -3], 1), -1);
}
