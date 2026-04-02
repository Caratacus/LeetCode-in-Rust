// Tests for Problem 0153: Find Minimum in Rotated Sorted Array
// Java reference: src/test/java/g0121_0200/s0153_find_minimum_in_rotated_sorted_array/SolutionTest.java

use leetcode_in_rust::s0153::find_minimum_in_rotated_sorted_array::Solution;

#[test]
fn test_find_min() {
    assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
}

#[test]
fn test_find_min2() {
    assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
}

#[test]
fn test_find_min3() {
    assert_eq!(Solution::find_min(vec![11, 13, 15, 17]), 11);
}
