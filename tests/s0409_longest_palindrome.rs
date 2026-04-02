// Tests for Problem 0409: Longest Palindrome
// Java reference: src/test/java/g0401_0500/s0409_longest_palindrome/SolutionTest.java

use leetcode_in_rust::s0409::longest_palindrome::Solution;

#[test]
fn test_longest_palindrome() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
}

#[test]
fn test_longest_palindrome2() {
    assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
}

#[test]
fn test_longest_palindrome3() {
    assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
}
