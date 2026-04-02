// Tests for Problem 0300: Longest Increasing Subsequence
// Java reference: src/test/java/g0201_0300/s0300_longest_increasing_subsequence/SolutionTest.java

use leetcode_in_rust::s0300::longest_increasing_subsequence::Solution;

#[test]
fn test_length_of_lis() {
    assert_eq!(Solution::length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
}

#[test]
fn test_length_of_lis2() {
    assert_eq!(Solution::length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
}

#[test]
fn test_length_of_lis3() {
    assert_eq!(Solution::length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
}
