// Tests for Problem 0995: Minimum Number of K Consecutive Bit Flips
// Java reference: src/test/java/g0901_1000/s0995_minimum_number_of_k_consecutive_bit_flips/SolutionTest.java

use leetcode_in_rust::s0995::minimum_number_of_k_consecutive_bit_flips::Solution;

#[test]
fn test_min_k_bit_flips() {
    assert_eq!(Solution::min_k_bit_flips(vec![0, 1, 0], 1), 2);
}

#[test]
fn test_min_k_bit_flips2() {
    assert_eq!(Solution::min_k_bit_flips(vec![1, 1, 0], 2), -1);
}

#[test]
fn test_min_k_bit_flips3() {
    assert_eq!(
        Solution::min_k_bit_flips(vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
        3
    );
}
