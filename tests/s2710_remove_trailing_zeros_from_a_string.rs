// Tests for Problem 2710: Remove Trailing Zeros From a String
// Java reference: src/test/java/g2701_2800/s2710_remove_trailing_zeros_from_a_string/SolutionTest.java

use leetcode_in_rust::s2710::remove_trailing_zeros_from_a_string::Solution;

#[test]
fn test_remove_trailing_zeros() {
    assert_eq!(Solution::remove_trailing_zeros("51230100".to_string()), "512301");
}

#[test]
fn test_remove_trailing_zeros2() {
    assert_eq!(Solution::remove_trailing_zeros("123".to_string()), "123");
}
