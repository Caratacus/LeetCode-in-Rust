// Tests for Problem 0594: Longest Harmonious Subsequence
// Java reference: src/test/java/g0501_0600/s0594_longest_harmonious_subsequence/SolutionTest.java

use leetcode_in_rust::s0594::longest_harmonious_subsequence::Solution;

#[test]
fn test_find_lhs() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
}

#[test]
fn test_find_lhs2() {
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
}

#[test]
fn test_find_lhs3() {
    assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
}
