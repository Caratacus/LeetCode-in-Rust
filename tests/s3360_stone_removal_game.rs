// Tests for Problem 3360: Stone Removal Game
// Java reference: src/test/java/g3301_3400/s3360_stone_removal_game/SolutionTest.java

use leetcode_in_rust::s3360::stone_removal_game::Solution;

#[test]
fn test_can_alice_win() {
    assert_eq!(Solution::can_alice_win(12), true);
}

#[test]
fn test_can_alice_win2() {
    assert_eq!(Solution::can_alice_win(1), false);
}

#[test]
fn test_can_alice_win3() {
    assert_eq!(Solution::can_alice_win(19), false);
}
