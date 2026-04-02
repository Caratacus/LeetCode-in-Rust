// Tests for Problem 0009: Palindrome Number
// Java reference: src/test/java/g0001_0100/s0009_palindrome_number/SolutionTest.java

use leetcode_in_rust::s0009::palindrome_number::Solution;

#[test]
fn test_is_palindrome() {
    assert_eq!(Solution::is_palindrome(121), true);
}

#[test]
fn test_is_palindrome2() {
    assert_eq!(Solution::is_palindrome(-121), false);
}

#[test]
fn test_is_palindrome3() {
    assert_eq!(Solution::is_palindrome(10), false);
}
