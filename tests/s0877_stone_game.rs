// Tests for Problem 0877: Stone Game
// Java reference: src/test/java/g0801_0900/s0877_stone_game/SolutionTest.java

use leetcode_in_rust::s0877::stone_game::Solution;

#[test]
fn test_stone_game() {
    assert_eq!(Solution::stone_game(vec![5, 3, 4, 5]), true);
}

#[test]
fn test_stone_game2() {
    assert_eq!(Solution::stone_game(vec![3, 7, 2, 3]), true);
}
