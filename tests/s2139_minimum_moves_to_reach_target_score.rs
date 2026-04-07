// Tests for Problem 2139: Minimum Moves to Reach Target Score
// Java reference: src/test/java/g2101_2200/s2139_minimum_moves_to_reach_target_score/SolutionTest.java

use leetcode_in_rust::s2139::minimum_moves_to_reach_target_score::Solution;

#[test]
fn test_min_moves() {
    assert_eq!(Solution::min_moves(5, 0), 4);
}

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves(19, 2), 7);
}

#[test]
fn test_min_moves3() {
    assert_eq!(Solution::min_moves(10, 4), 4);
}
