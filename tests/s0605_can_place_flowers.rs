// Tests for Problem 0605: Can Place Flowers
// Java reference: src/test/java/g0601_0700/s0605_can_place_flowers/SolutionTest.java

use leetcode_in_rust::s0605::can_place_flowers::Solution;

#[test]
fn test_can_place_flowers() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1), true);
}

#[test]
fn test_can_place_flowers2() {
    assert_eq!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2), false);
}
