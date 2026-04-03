// Tests for Problem 1328: Break a Palindrome
// Java reference: src/test/java/g1301_1400/s1328_break_a_palindrome/SolutionTest.java

use leetcode_in_rust::s1328::break_a_palindrome::Solution;

#[test]
fn test_break_palindrome() {
    assert_eq!(Solution::break_palindrome("abccba".to_string()), "aaccba");
}

#[test]
fn test_break_palindrome2() {
    assert_eq!(Solution::break_palindrome("a".to_string()), "");
}
