// Tests for Problem 0055: Jump Game
// Java reference: src/test/java/g0001_0100/s0055_jump_game/SolutionTest.java

use leetcode_in_rust::s0055::jump_game::Solution;

#[test]
fn test_can_jump() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
}

#[test]
fn test_can_jump2() {
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}
