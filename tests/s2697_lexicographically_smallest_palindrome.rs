// Tests for Problem 2697: Lexicographically Smallest Palindrome
// Java reference: src/test/java/g2601_2700/s2697_lexicographically_smallest_palindrome/SolutionTest.java

use leetcode_in_rust::s2697::lexicographically_smallest_palindrome::Solution;

#[test]
fn test_make_smallest_palindrome() {
    assert_eq!(Solution::make_smallest_palindrome("egcfe".to_string()), "efcfe");
}

#[test]
fn test_make_smallest_palindrome2() {
    assert_eq!(Solution::make_smallest_palindrome("abcd".to_string()), "abba");
}

#[test]
fn test_make_smallest_palindrome3() {
    assert_eq!(Solution::make_smallest_palindrome("seven".to_string()), "neven");
}
