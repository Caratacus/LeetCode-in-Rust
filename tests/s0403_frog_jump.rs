// Tests for Problem 0403: Frog Jump
// Java reference: src/test/java/g0401_0500/s0403_frog_jump/SolutionTest.java

use leetcode_in_rust::s0403::frog_jump::Solution;

#[test]
fn test_can_cross() {
    assert_eq!(Solution::can_cross(vec![0, 1, 3, 5, 6, 8, 12, 17]), true);
}

#[test]
fn test_can_cross2() {
    assert_eq!(Solution::can_cross(vec![0, 1, 2, 3, 4, 8, 9, 11]), false);
}
