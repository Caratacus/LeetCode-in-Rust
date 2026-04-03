// Tests for Problem 0810: Chalkboard XOR Game
// Java reference: src/test/java/g0701_0800/s0810_chalkboard_xor_game/SolutionTest.java

use leetcode_in_rust::s0810::chalkboard_xor_game::Solution;

#[test]
fn test_xor_game() {
    assert_eq!(Solution::xor_game(vec![1, 1, 2]), false);
}

#[test]
fn test_xor_game2() {
    assert_eq!(Solution::xor_game(vec![0, 1]), true);
}

#[test]
fn test_xor_game3() {
    assert_eq!(Solution::xor_game(vec![1, 2, 3]), true);
}
