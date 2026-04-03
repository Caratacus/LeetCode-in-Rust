// Tests for Problem 0673: Number of Longest Increasing Subsequence
// Java reference: src/test/java/g0601_0700/s0673_number_of_longest_increasing_subsequence/SolutionTest.java

use leetcode_in_rust::s0673::number_of_longest_increasing_subsequence::Solution;

#[test]
fn test_find_number_of_lis() {
    assert_eq!(Solution::find_number_of_lis(vec![1, 3, 5, 4, 7]), 2);
}

#[test]
fn test_find_number_of_lis2() {
    assert_eq!(Solution::find_number_of_lis(vec![2, 2, 2, 2, 2]), 5);
}
