// Tests for Problem 1690: Stone Game VII
// Java reference: src/test/java/g1601_1700/s1690_stone_game_vii/SolutionTest.java

use leetcode_in_rust::s1690::stone_game_vii::Solution;

#[test]
fn test_stone_game_vii() {
    assert_eq!(Solution::stone_game_vii(vec![5, 3, 1, 4, 2]), 6);
}

#[test]
fn test_stone_game_vii2() {
    assert_eq!(Solution::stone_game_vii(vec![7, 90, 5, 1, 100, 10, 10, 2]), 122);
}
