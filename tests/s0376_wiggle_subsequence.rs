// Tests for Problem 0376: Wiggle Subsequence
// Java reference: src/test/java/g0301_0400/s0376_wiggle_subsequence/SolutionTest.java

use leetcode_in_rust::s0376::wiggle_subsequence::Solution;

#[test]
fn test_wiggle_max_length() {
    assert_eq!(Solution::wiggle_max_length(vec![1, 7, 4, 9, 2, 5]), 6);
}

#[test]
fn test_wiggle_max_length2() {
    assert_eq!(Solution::wiggle_max_length(vec![1, 17, 5, 10, 13, 15, 10, 5, 16, 8]), 7);
}

#[test]
fn test_wiggle_max_length3() {
    assert_eq!(Solution::wiggle_max_length(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 2);
}
