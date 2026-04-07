// Tests for Problem 2472: Maximum Number of Non-overlapping Palindrome Substrings
// Java reference: src/test/java/g2401_2500/s2472_maximum_number_of_non_overlapping_palindrome_substrings/SolutionTest.java

use leetcode_in_rust::s2472::maximum_number_of_non_overlapping_palindrome_substrings::Solution;

#[test]
fn test_max_palindromes() {
    assert_eq!(Solution::max_palindromes("abaccdbbd".to_string(), 3), 2);
}

#[test]
fn test_max_palindromes2() {
    assert_eq!(Solution::max_palindromes("adbcda".to_string(), 2), 0);
}
