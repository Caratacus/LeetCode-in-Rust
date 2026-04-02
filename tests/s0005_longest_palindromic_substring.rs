// Tests for Problem 0005: Longest Palindromic Substring
// Java reference: src/test/java/g0001_0100/s0005_longest_palindromic_substring/SolutionTest.java

use leetcode_in_rust::s0005::longest_palindromic_substring::Solution;

#[test]
fn test_longest_palindrome() {
    let result = Solution::longest_palindrome("babad".to_string());
    assert!(result == "bab" || result == "aba");
}

#[test]
fn test_longest_palindrome2() {
    assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
}
