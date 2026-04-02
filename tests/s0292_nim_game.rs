// Tests for Problem 0292: Nim Game
// Java reference: src/test/java/g0201_0300/s0292_nim_game/SolutionTest.java

use leetcode_in_rust::s0292::nim_game::Solution;

#[test]
fn test_can_win_nim() {
    assert_eq!(Solution::can_win_nim(4), false);
}

#[test]
fn test_can_win_nim2() {
    assert_eq!(Solution::can_win_nim(1), true);
}

#[test]
fn test_can_win_nim3() {
    assert_eq!(Solution::can_win_nim(2), true);
}
