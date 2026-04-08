// Tests for Problem 3222: Find the Winning Player in Coin Game
// Java reference: src/test/java/g3201_3300/s3222_find_the_winning_player_in_coin_game/SolutionTest.java

use leetcode_in_rust::s3222::find_the_winning_player_in_coin_game::Solution;

#[test]
fn test_losing_player() {
    assert_eq!(Solution::losing_player(2, 7), "Alice");
}

#[test]
fn test_losing_player2() {
    assert_eq!(Solution::losing_player(4, 11), "Bob");
}
