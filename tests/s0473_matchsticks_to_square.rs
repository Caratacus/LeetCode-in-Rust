// Tests for Problem 0473: Matchsticks to Square
// Java reference: src/test/java/g0401_0500/s0473_matchsticks_to_square/SolutionTest.java

use leetcode_in_rust::s0473::matchsticks_to_square::Solution;

#[test]
fn test_makesquare() {
    assert_eq!(Solution::makesquare(vec![1, 1, 2, 2, 2]), true);
}

#[test]
fn test_makesquare2() {
    assert_eq!(Solution::makesquare(vec![3, 3, 3, 3, 4]), false);
}
