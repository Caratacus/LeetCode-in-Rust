// Tests for Problem 1027: Longest Arithmetic Subsequence
// Java reference: src/test/java/g1001_1100/s1027_longest_arithmetic_subsequence/SolutionTest.java

use leetcode_in_rust::s1027::longest_arithmetic_subsequence::Solution;

#[test]
fn test_longest_arith_seq_length() {
    assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
}

#[test]
fn test_longest_arith_seq_length2() {
    assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
}

#[test]
fn test_longest_arith_seq_length3() {
    assert_eq!(Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]), 4);
}
