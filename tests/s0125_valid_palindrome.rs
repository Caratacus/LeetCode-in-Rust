// Tests for Problem 0125: Valid Palindrome
// Java reference: src/test/java/g0121_0200/s0125_valid_palindrome/SolutionTest.java

use leetcode_in_rust::s0125::valid_palindrome::Solution;

#[test]
fn test_is_palindrome() {
    assert_eq!(Solution::is_palindrome(String::from("A man, a plan, a canal: Panama")), true);
}

#[test]
fn test_is_palindrome2() {
    assert_eq!(Solution::is_palindrome(String::from("race a car")), false);
}

#[test]
fn test_is_palindrome3() {
    assert_eq!(Solution::is_palindrome(String::from(" ")), true);
}
