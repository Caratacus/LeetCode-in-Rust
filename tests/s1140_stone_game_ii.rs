// Tests for Problem 1140: Stone Game II
// Java reference: src/test/java/g1101_1200/s1140_stone_game_ii/SolutionTest.java

use leetcode_in_rust::s1140::stone_game_ii::Solution;

#[test]
fn test_stone_game_ii() {
    assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
}

#[test]
fn test_stone_game_ii2() {
    assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
}
