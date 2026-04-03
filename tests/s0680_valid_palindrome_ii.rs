// Tests for Problem 0680: Valid Palindrome II
// Java reference: src/test/java/g0601_0700/s0680_valid_palindrome_ii/SolutionTest.java

use leetcode_in_rust::s0680::valid_palindrome_ii::Solution;

#[test]
fn test_valid_palindrome() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
}

#[test]
fn test_valid_palindrome2() {
    assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
}

#[test]
fn test_valid_palindrome3() {
    assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
}
