// Tests for Problem 1326: Minimum Number of Taps to Open to Water a Garden
// Java reference: src/test/java/g1301_1400/s1326_minimum_number_of_taps_to_open_to_water_a_garden/SolutionTest.java

use leetcode_in_rust::s1326::minimum_number_of_taps_to_open_to_water_a_garden::Solution;

#[test]
fn test_min_taps() {
    assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
}

#[test]
fn test_min_taps2() {
    assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
}
