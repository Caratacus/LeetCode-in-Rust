// Tests for Problem 1681: Minimum Incompatibility
// Java reference: src/test/java/g1601_1700/s1681_minimum_incompatibility/SolutionTest.java

use leetcode_in_rust::s1681::minimum_incompatibility::Solution;

#[test]
fn test_minimum_incompatibility() {
    assert_eq!(Solution::minimum_incompatibility(vec![1, 2, 1, 4], 2), 4);
}

#[test]
fn test_minimum_incompatibility2() {
    assert_eq!(Solution::minimum_incompatibility(vec![6, 3, 8, 1, 3, 1, 2, 2], 4), 6);
}

#[test]
fn test_minimum_incompatibility3() {
    assert_eq!(Solution::minimum_incompatibility(vec![5, 3, 3, 6, 3, 3], 3), -1);
}
