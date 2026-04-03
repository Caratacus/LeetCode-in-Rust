// Tests for Problem 1686: Stone Game VI
// Java reference: src/test/java/g1601_1700/s1686_stone_game_vi/SolutionTest.java

use leetcode_in_rust::s1686::stone_game_vi::Solution;

#[test]
fn test_stone_game_vi() {
    assert_eq!(Solution::stone_game_vi(vec![1, 3], vec![2, 1]), 1);
}

#[test]
fn test_stone_game_vi2() {
    assert_eq!(Solution::stone_game_vi(vec![1, 2], vec![3, 1]), 0);
}

#[test]
fn test_stone_game_vi3() {
    assert_eq!(Solution::stone_game_vi(vec![2, 4, 3], vec![1, 6, 7]), -1);
}
