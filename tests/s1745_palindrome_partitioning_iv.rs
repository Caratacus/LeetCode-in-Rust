// Tests for Problem 1745: Palindrome Partitioning IV
// Java reference: src/test/java/g1701_1800/s1745_palindrome_partitioning_iv/SolutionTest.java

use leetcode_in_rust::s1745::palindrome_partitioning_iv::Solution;

#[test]
fn test_check_partitioning() {
    assert_eq!(Solution::check_partitioning("abcbdd".to_string()), true);
}

#[test]
fn test_check_partitioning2() {
    assert_eq!(Solution::check_partitioning("bcbddxy".to_string()), false);
}
