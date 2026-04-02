// Tests for Problem 0132: Palindrome Partitioning II
// Java reference: src/test/java/g0121_0200/s0132_palindrome_partitioning_ii/SolutionTest.java

use leetcode_in_rust::s0132::palindrome_partitioning_ii::Solution;

#[test]
fn test_min_cut() {
    assert_eq!(Solution::min_cut(String::from("aab")), 1);
}

#[test]
fn test_min_cut2() {
    assert_eq!(Solution::min_cut(String::from("a")), 0);
}

#[test]
fn test_min_cut3() {
    assert_eq!(Solution::min_cut(String::from("ab")), 1);
}
