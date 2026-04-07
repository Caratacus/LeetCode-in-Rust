// Tests for Problem 2511: Maximum Enemy Forts That Can Be Captured
// Java reference: src/test/java/g2401_2500/s2511_maximum_enemy_forts_that_can_be_captured/SolutionTest.java

use leetcode_in_rust::s2511::maximum_enemy_forts_that_can_be_captured::Solution;

#[test]
fn test_capture_forts() {
    assert_eq!(Solution::capture_forts(vec![1, 0, 0, -1, 0, 0, 0, 1]), 4);
}

#[test]
fn test_capture_forts2() {
    assert_eq!(Solution::capture_forts(vec![0, 0, 1, -1]), 0);
}
