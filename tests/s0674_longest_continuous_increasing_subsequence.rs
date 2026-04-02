// Tests for Problem 0674: Longest Continuous Increasing Subsequence
// Java reference: src/test/java/g0601_0700/s0674_longest_continuous_increasing_subsequence/SolutionTest.java

use leetcode_in_rust::s0674::longest_continuous_increasing_subsequence::Solution;

#[test]
fn test_find_length_of_lcis() {
    assert_eq!(Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
}

#[test]
fn test_find_length_of_lcis2() {
    assert_eq!(Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2]), 1);
}
