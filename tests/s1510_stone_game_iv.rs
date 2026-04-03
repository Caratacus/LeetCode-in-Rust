// Tests for Problem 1510: Stone Game IV
// Java reference: src/test/java/g1501_1600/s1510_stone_game_iv/SolutionTest.java

use leetcode_in_rust::s1510::stone_game_iv::Solution;

#[test]
fn test_winner_square_game() {
    assert_eq!(Solution::winner_square_game(1), true);
}

#[test]
fn test_winner_square_game2() {
    assert_eq!(Solution::winner_square_game(2), false);
}

#[test]
fn test_winner_square_game3() {
    assert_eq!(Solution::winner_square_game(4), true);
}
