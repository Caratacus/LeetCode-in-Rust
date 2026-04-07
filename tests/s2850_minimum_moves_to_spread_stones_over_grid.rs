// Tests for Problem 2850: Minimum Moves to Spread Stones Over Grid
// Java reference: src/test/java/g2801_2900/s2850_minimum_moves_to_spread_stones_over_grid/SolutionTest.java

use leetcode_in_rust::s2850::minimum_moves_to_spread_stones_over_grid::Solution;

#[test]
fn test_minimum_moves() {
    assert_eq!(
        Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]),
        3
    );
}

#[test]
fn test_minimum_moves2() {
    assert_eq!(
        Solution::minimum_moves(vec![vec![1, 3, 0], vec![1, 0, 0], vec![1, 0, 3]]),
        4
    );
}
