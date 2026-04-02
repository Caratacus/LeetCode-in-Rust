// Tests for Problem 0647: Palindromic Substrings
// Java reference: src/test/java/g0601_0700/s0647_palindromic_substrings/SolutionTest.java

use leetcode_in_rust::s0647::palindromic_substrings::Solution;

#[test]
fn test_count_substrings() {
    assert_eq!(Solution::count_substrings("abc".to_string()), 3);
}

#[test]
fn test_count_substrings2() {
    assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
}
