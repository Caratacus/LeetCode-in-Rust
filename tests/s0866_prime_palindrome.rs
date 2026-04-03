// Tests for Problem 0866: Prime Palindrome
// Java reference: src/test/java/g0801_0900/s0866_prime_palindrome/SolutionTest.java

use leetcode_in_rust::s0866::prime_palindrome::Solution;

#[test]
fn test_prime_palindrome() {
    assert_eq!(Solution::prime_palindrome(6), 7);
}

#[test]
fn test_prime_palindrome2() {
    assert_eq!(Solution::prime_palindrome(8), 11);
}

#[test]
fn test_prime_palindrome3() {
    assert_eq!(Solution::prime_palindrome(13), 101);
}
