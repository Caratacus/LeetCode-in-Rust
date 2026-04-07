// Tests for Problem 2808: Minimum Seconds to Equalize a Circular Array
// Java reference: src/test/java/g2701_2800/s2808_minimum_seconds_to_equalize_a_circular_array/SolutionTest.java

use leetcode_in_rust::s2808::minimum_seconds_to_equalize_a_circular_array::Solution;

#[test]
fn test_minimum_seconds() {
    assert_eq!(Solution::minimum_seconds(vec![2, 1, 3, 3, 2]), 2);
}

#[test]
fn test_minimum_seconds2() {
    assert_eq!(Solution::minimum_seconds(vec![5, 5, 5, 5]), 0);
}
