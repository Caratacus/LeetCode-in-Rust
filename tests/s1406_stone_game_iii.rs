// Tests for Problem 1406: Stone Game III
// Java reference: src/test/java/g1301_1400/s1406_stone_game_iii/SolutionTest.java

use leetcode_in_rust::s1406::stone_game_iii::Solution;

#[test]
fn test_stone_game_iii() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 7]), "Bob");
}

#[test]
fn test_stone_game_iii2() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, -9]), "Alice");
}

#[test]
fn test_stone_game_iii3() {
    assert_eq!(Solution::stone_game_iii(vec![1, 2, 3, 6]), "Tie");
}
