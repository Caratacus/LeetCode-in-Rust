// Tests for Problem 0453: Minimum Moves to Equal Array Elements
// Java reference: src/test/java/g0401_0500/s0453_minimum_moves_to_equal_array_elements/SolutionTest.java

use leetcode_in_rust::s0453::minimum_moves_to_equal_array_elements::Solution;

#[test]
fn test_min_moves() {
    assert_eq!(Solution::min_moves(vec![1, 2, 3]), 3);
}

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves(vec![1, 1, 1]), 0);
}
