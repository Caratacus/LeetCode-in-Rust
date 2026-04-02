// Tests for Problem 0072: Edit Distance
// Java reference: src/test/java/g0001_0100/s0072_edit_distance/SolutionTest.java

use leetcode_in_rust::s0072::edit_distance::Solution;

#[test]
fn test_min_distance() {
    assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
}

#[test]
fn test_min_distance2() {
    assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
}
