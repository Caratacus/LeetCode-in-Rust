// Tests for Problem 0076: Minimum Window Substring
// Java reference: src/test/java/g0001_0100/s0076_minimum_window_substring/SolutionTest.java

use leetcode_in_rust::s0076::minimum_window_substring::Solution;

#[test]
fn test_min_window() {
    assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
}

#[test]
fn test_min_window2() {
    assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
}

#[test]
fn test_min_window3() {
    assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
}
