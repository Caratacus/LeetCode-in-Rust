// Tests for Problem 0390: Elimination Game
// Java reference: src/test/java/g0301_0400/s0390_elimination_game/SolutionTest.java

use leetcode_in_rust::s0390::elimination_game::Solution;

#[test]
fn test_last_remaining() {
    assert_eq!(Solution::last_remaining(9), 6);
}

#[test]
fn test_last_remaining2() {
    assert_eq!(Solution::last_remaining(1), 1);
}
