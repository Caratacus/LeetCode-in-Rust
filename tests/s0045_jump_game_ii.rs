// Tests for Problem 0045: Jump Game II
// Java reference: src/test/java/g0001_0100/s0045_jump_game_ii/SolutionTest.java

use leetcode_in_rust::s0045::jump_game_ii::Solution;

#[test]
fn test_jump() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
}

#[test]
fn test_jump2() {
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
}
