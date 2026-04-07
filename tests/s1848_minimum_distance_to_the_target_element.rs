// Tests for Problem 1848: Minimum Distance to the Target Element
// Java reference: src/test/java/g1801_1900/s1848_minimum_distance_to_the_target_element/SolutionTest.java

use leetcode_in_rust::s1848::minimum_distance_to_the_target_element::Solution;

#[test]
fn test_get_min_distance() {
    assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
}

#[test]
fn test_get_min_distance2() {
    assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
}

#[test]
fn test_get_min_distance3() {
    assert_eq!(
        Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 3),
        0
    );
}
