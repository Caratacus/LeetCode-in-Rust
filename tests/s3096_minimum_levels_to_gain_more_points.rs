// Tests for Problem 3096: Minimum Levels to Gain More Points
// Java reference: src/test/java/g3001_3100/s3096_minimum_levels_to_gain_more_points/SolutionTest.java

use leetcode_in_rust::s3096::minimum_levels_to_gain_more_points::Solution;

#[test]
fn test_minimum_levels() {
    assert_eq!(Solution::minimum_levels(vec![1, 0, 1, 0]), 1);
}

#[test]
fn test_minimum_levels2() {
    assert_eq!(Solution::minimum_levels(vec![1, 1, 1, 1, 1]), 3);
}

#[test]
fn test_minimum_levels3() {
    assert_eq!(Solution::minimum_levels(vec![0, 0]), -1);
}
