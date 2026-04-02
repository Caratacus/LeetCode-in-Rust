// Tests for Problem 1025: Divisor Game
// Java reference: src/test/java/g1001_1100/s1025_divisor_game/SolutionTest.java

use leetcode_in_rust::s1025::divisor_game::Solution;

#[test]
fn test_divisor_game() {
    assert_eq!(Solution::divisor_game(2), true);
}

#[test]
fn test_divisor_game2() {
    assert_eq!(Solution::divisor_game(3), false);
}
