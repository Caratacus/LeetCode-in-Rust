// Tests for Problem 0214: Shortest Palindrome
// Java reference: src/test/java/g0201_0300/s0214_shortest_palindrome/SolutionTest.java

use leetcode_in_rust::s0214::shortest_palindrome::Solution;

#[test]
fn test_shortest_palindrome() {
    assert_eq!(Solution::shortest_palindrome("aacecaaa".to_string()), "aaacecaaa");
}

#[test]
fn test_shortest_palindrome2() {
    assert_eq!(Solution::shortest_palindrome("abcd".to_string()), "dcbabcd");
}
