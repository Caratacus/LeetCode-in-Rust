// Tests for Problem 0479: Largest Palindrome Product
// Java reference: src/test/java/g0401_0500/s0479_largest_palindrome_product/SolutionTest.java

use leetcode_in_rust::s0479::largest_palindrome_product::Solution;

#[test]
fn test_largest_palindrome() {
    assert_eq!(Solution::largest_palindrome(2), 987);
}

#[test]
fn test_largest_palindrome2() {
    assert_eq!(Solution::largest_palindrome(1), 9);
}
