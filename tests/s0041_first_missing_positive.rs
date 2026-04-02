// Tests for Problem 0041: First Missing Positive
// Java reference: src/test/java/g0001_0100/s0041_first_missing_positive/SolutionTest.java

use leetcode_in_rust::s0041::first_missing_positive::Solution;

#[test]
fn test_first_missing_positive() {
    assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
}

#[test]
fn test_first_missing_positive2() {
    assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
}

#[test]
fn test_first_missing_positive3() {
    assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
}
