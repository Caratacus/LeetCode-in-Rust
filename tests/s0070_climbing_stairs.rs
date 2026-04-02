// Tests for Problem 0070: Climbing Stairs
// Java reference: src/test/java/g0001_0100/s0070_climbing_stairs/SolutionTest.java

use leetcode_in_rust::s0070::climbing_stairs::Solution;

#[test]
fn test_climb_stairs() {
    assert_eq!(Solution::climb_stairs(2), 2);
}

#[test]
fn test_climb_stairs2() {
    assert_eq!(Solution::climb_stairs(3), 3);
}

#[test]
fn test_climb_stairs3() {
    assert_eq!(Solution::climb_stairs(1), 1);
}
