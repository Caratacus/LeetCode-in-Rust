// Tests for Problem 0801: Minimum Swaps to Make Sequences Increasing
// Java reference: src/test/java/g0801_0900/s0801_minimum_swaps_to_make_sequences_increasing/SolutionTest.java

use leetcode_in_rust::s0801::minimum_swaps_to_make_sequences_increasing::Solution;

#[test]
fn test_min_swap() {
    assert_eq!(Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]), 1);
}

#[test]
fn test_min_swap2() {
    assert_eq!(Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]), 1);
}
