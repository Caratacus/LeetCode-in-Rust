// Tests for Problem 1526: Minimum Number of Increments on Subarrays to Form a Target Array
// Java reference: src/test/java/g1501_1600/s1526_minimum_number_of_increments_on_subarrays_to_form_a_target_array/SolutionTest.java

use leetcode_in_rust::s1526::minimum_number_of_increments_on_subarrays_to_form_a_target_array::Solution;

#[test]
fn test_min_number_operations() {
    assert_eq!(Solution::min_number_operations(vec![1, 2, 3, 2, 1]), 3);
}

#[test]
fn test_min_number_operations2() {
    assert_eq!(Solution::min_number_operations(vec![3, 1, 1, 2]), 4);
}

#[test]
fn test_min_number_operations3() {
    assert_eq!(Solution::min_number_operations(vec![3, 1, 5, 4, 2]), 7);
}
