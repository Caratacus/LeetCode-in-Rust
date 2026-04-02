// Tests for Problem 0042: Trapping Rain Water
// Java reference: src/test/java/g0001_0100/s0042_trapping_rain_water/SolutionTest.java

use leetcode_in_rust::s0042::trapping_rain_water::Solution;

#[test]
fn test_trap() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
}

#[test]
fn test_trap2() {
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
