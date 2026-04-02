// Tests for Problem 0365: Water and Jug Problem
// Java reference: src/test/java/g0301_0400/s0365_water_and_jug_problem/SolutionTest.java

use leetcode_in_rust::s0365::water_and_jug_problem::Solution;

#[test]
fn test_can_measure_water() {
    assert_eq!(Solution::can_measure_water(3, 5, 4), true);
}

#[test]
fn test_can_measure_water2() {
    assert_eq!(Solution::can_measure_water(2, 6, 5), false);
}

#[test]
fn test_can_measure_water3() {
    assert_eq!(Solution::can_measure_water(1, 2, 3), true);
}
