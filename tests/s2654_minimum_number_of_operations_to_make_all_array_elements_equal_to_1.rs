// Tests for Problem 2654: Minimum Number of Operations to Make All Array Elements Equal to 1
// Java reference: src/test/java/g2601_2700/s2654_minimum_number_of_operations_to_make_all_array_elements_equal_to_1/SolutionTest.java

use leetcode_in_rust::s2654::minimum_number_of_operations_to_make_all_array_elements_equal_to_1::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![2, 6, 3, 4]), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 10, 6, 14]), -1);
}
