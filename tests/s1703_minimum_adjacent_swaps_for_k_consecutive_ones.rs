// Tests for Problem 1703: Minimum Adjacent Swaps for K Consecutive Ones
// Java reference: src/test/java/g1701_1800/s1703_minimum_adjacent_swaps_for_k_consecutive_ones/SolutionTest.java

use leetcode_in_rust::s1703::minimum_adjacent_swaps_for_k_consecutive_ones::Solution;

#[test]
fn test_min_moves() {
    assert_eq!(Solution::min_moves(vec![1, 0, 0, 1, 0, 1], 2), 1);
}

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves(vec![1, 0, 0, 0, 0, 0, 1, 1], 3), 5);
}

#[test]
fn test_min_moves3() {
    assert_eq!(Solution::min_moves(vec![1, 1, 0, 1], 2), 0);
}
