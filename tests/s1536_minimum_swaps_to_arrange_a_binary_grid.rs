// Tests for Problem 1536: Minimum Swaps to Arrange a Binary Grid
// Java reference: src/test/java/g1501_1600/s1536_minimum_swaps_to_arrange_a_binary_grid/SolutionTest.java

use leetcode_in_rust::s1536::minimum_swaps_to_arrange_a_binary_grid::Solution;

#[test]
fn test_min_swaps() {
    assert_eq!(Solution::min_swaps(vec![vec![0, 0, 1], vec![1, 1, 0], vec![1, 0, 0]]), 3);
}

#[test]
fn test_min_swaps2() {
    assert_eq!(Solution::min_swaps(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 1, 1, 0]]), -1);
}

#[test]
fn test_min_swaps3() {
    assert_eq!(Solution::min_swaps(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]), 0);
}
