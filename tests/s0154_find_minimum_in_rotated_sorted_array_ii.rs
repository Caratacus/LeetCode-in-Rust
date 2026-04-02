// Tests for Problem 0154: Find Minimum in Rotated Sorted Array II
// Java reference: src/test/java/g0121_0200/s0154_find_minimum_in_rotated_sorted_array_ii/SolutionTest.java

use leetcode_in_rust::s0154::find_minimum_in_rotated_sorted_array_ii::Solution;

#[test]
fn test_find_min() {
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
}

#[test]
fn test_find_min2() {
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
}
