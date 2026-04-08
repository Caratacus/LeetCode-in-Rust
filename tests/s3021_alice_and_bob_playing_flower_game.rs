// Tests for Problem 3021: Alice and Bob Playing Flower Game
// Java reference: src/test/java/g3001_3100/s3021_alice_and_bob_playing_flower_game/SolutionTest.java

use leetcode_in_rust::s3021::alice_and_bob_playing_flower_game::Solution;

#[test]
fn test_flower_game() {
    assert_eq!(Solution::flower_game(3, 2), 3);
}

#[test]
fn test_flower_game2() {
    assert_eq!(Solution::flower_game(1, 1), 0);
}
