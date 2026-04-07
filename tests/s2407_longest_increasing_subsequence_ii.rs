// Tests for Problem 2407: Longest Increasing Subsequence II
// Java reference: src/test/java/g2401_2500/s2407_longest_increasing_subsequence_ii/SolutionTest.java

use leetcode_in_rust::s2407::longest_increasing_subsequence_ii::Solution;

#[test]
fn test_length_of_lis() {
    assert_eq!(Solution::length_of_lis(vec![4, 2, 1, 4, 3, 4, 5, 8, 15], 3), 5);
}

#[test]
fn test_length_of_lis2() {
    assert_eq!(Solution::length_of_lis(vec![7, 4, 5, 1, 8, 12, 4, 7], 5), 4);
}

#[test]
fn test_length_of_lis3() {
    assert_eq!(Solution::length_of_lis(vec![1, 5], 1), 1);
}
