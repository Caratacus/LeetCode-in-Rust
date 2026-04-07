// Tests for Problem 2934: Minimum Operations to Maximize Last Elements in Arrays
// Java reference: src/test/java/g2901_3000/s2934_minimum_operations_to_maximize_last_elements_in_arrays/SolutionTest.java

use leetcode_in_rust::s2934::minimum_operations_to_maximize_last_elements_in_arrays::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 2, 7], vec![4, 5, 3]), 1);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 3, 4, 5, 9], vec![8, 8, 4, 4, 4]), 2);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![1, 5, 4], vec![2, 5, 3]), -1);
}
