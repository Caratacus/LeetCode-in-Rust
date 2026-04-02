// Tests for Problem 0540: Single Element in a Sorted Array
// Java reference: src/test/java/g0501_0600/s0540_single_element_in_a_sorted_array/SolutionTest.java

use leetcode_in_rust::s0540::single_element_in_a_sorted_array::Solution;

#[test]
fn test_single_non_duplicate() {
    assert_eq!(Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]), 2);
}

#[test]
fn test_single_non_duplicate2() {
    assert_eq!(Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]), 10);
}
