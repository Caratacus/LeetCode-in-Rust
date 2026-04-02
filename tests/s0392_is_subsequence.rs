// Tests for Problem 0392: Is Subsequence
// Java reference: src/test/java/g0301_0400/s0392_is_subsequence/SolutionTest.java

use leetcode_in_rust::s0392::is_subsequence::Solution;

#[test]
fn test_is_subsequence() {
    assert_eq!(Solution::is_subsequence("abc".to_string(), "ahbgdc".to_string()), true);
}

#[test]
fn test_is_subsequence2() {
    assert_eq!(Solution::is_subsequence("axc".to_string(), "ahbgdc".to_string()), false);
}
