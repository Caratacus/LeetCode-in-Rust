// Tests for Problem 0650: 2 Keys Keyboard
// Java reference: src/test/java/g0601_0700/s0650_2_keys_keyboard/SolutionTest.java

use leetcode_in_rust::s0650::p2_keys_keyboard::Solution;

#[test]
fn test_min_steps() {
    assert_eq!(Solution::min_steps(3), 3);
}

#[test]
fn test_min_steps2() {
    assert_eq!(Solution::min_steps(1), 0);
}
