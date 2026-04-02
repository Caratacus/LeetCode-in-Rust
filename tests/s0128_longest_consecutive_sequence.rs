// Tests for Problem 0128: Longest Consecutive Sequence
// Java reference: src/test/java/g0121_0200/s0128_longest_consecutive_sequence/SolutionTest.java

use leetcode_in_rust::s0128::longest_consecutive_sequence::Solution;

#[test]
fn test_longest_consecutive() {
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
}

#[test]
fn test_longest_consecutive2() {
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
}

#[test]
fn test_longest_consecutive3() {
    assert_eq!(Solution::longest_consecutive(vec![]), 0);
}
