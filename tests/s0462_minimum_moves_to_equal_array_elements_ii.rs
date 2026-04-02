// Tests for Problem 0462: Minimum Moves to Equal Array Elements II
// Java reference: src/test/java/g0401_0500/s0462_minimum_moves_to_equal_array_elements_ii/SolutionTest.java

use leetcode_in_rust::s0462::minimum_moves_to_equal_array_elements_ii::Solution;

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves2(vec![1, 2, 3]), 2);
}

#[test]
fn test_min_moves22() {
    assert_eq!(Solution::min_moves2(vec![1, 10, 2, 9]), 16);
}
