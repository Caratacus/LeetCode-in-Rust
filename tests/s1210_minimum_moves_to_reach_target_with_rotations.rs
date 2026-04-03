// Tests for Problem 1210: Minimum Moves to Reach Target with Rotations
// Java reference: src/test/java/g1201_1300/s1210_minimum_moves_to_reach_target_with_rotations/SolutionTest.java

use leetcode_in_rust::s1210::minimum_moves_to_reach_target_with_rotations::Solution;

#[test]
fn test_minimum_moves() {
    let result = Solution::minimum_moves(vec![
        vec![0, 0, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 1, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0],
    ]);
    assert_eq!(result, 11);
}

#[test]
fn test_minimum_moves2() {
    let result = Solution::minimum_moves(vec![
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 0, 1],
        vec![1, 1, 1, 0, 0, 0],
    ]);
    assert_eq!(result, 9);
}
