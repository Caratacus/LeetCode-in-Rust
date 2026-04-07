// Tests for Problem 2208: Minimum Operations to Halve Array Sum
// Java reference: src/test/java/g2201_2300/s2208_minimum_operations_to_halve_array_sum/SolutionTest.java

use leetcode_in_rust::s2208::minimum_operations_to_halve_array_sum::Solution;

#[test]
fn test_halve_array() {
    assert_eq!(Solution::halve_array(vec![5, 19, 8, 1]), 3);
}

#[test]
fn test_halve_array2() {
    assert_eq!(Solution::halve_array(vec![3, 8, 20]), 3);
}
