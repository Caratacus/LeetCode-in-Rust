// Tests for Problem 1403: Minimum Subsequence in Non-Increasing Order
// Java reference: src/test/java/g1401_1500/s1403_minimum_subsequence_in_non_increasing_order/SolutionTest.java

use leetcode_in_rust::s1403::minimum_subsequence_in_non_increasing_order::Solution;

#[test]
fn test_min_subsequence() {
    assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
}

#[test]
fn test_min_subsequence2() {
    assert_eq!(Solution::min_subsequence(vec![4, 4, 7, 6, 7]), vec![7, 7, 6]);
}

#[test]
fn test_min_subsequence3() {
    assert_eq!(Solution::min_subsequence(vec![6]), vec![6]);
}
