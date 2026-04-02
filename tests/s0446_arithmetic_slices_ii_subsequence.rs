// Tests for Problem 0446: Arithmetic Slices II Subsequence
// Java reference: src/test/java/g0401_0500/s0446_arithmetic_slices_ii_subsequence/SolutionTest.java

use leetcode_in_rust::s0446::arithmetic_slices_ii_subsequence::Solution;

#[test]
fn test_number_of_arithmetic_slices() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![2, 4, 6, 8, 10]), 7);
}

#[test]
fn test_number_of_arithmetic_slices2() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![7, 7, 7, 7, 7]), 16);
}
