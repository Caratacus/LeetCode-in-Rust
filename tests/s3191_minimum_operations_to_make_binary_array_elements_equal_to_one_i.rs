// Tests for Problem 3191: Minimum Operations to Make Binary Array Elements Equal to One I
// Java reference: src/test/java/g3101_3200/s3191_minimum_operations_to_make_binary_array_elements_equal_to_one_i/SolutionTest.java

use leetcode_in_rust::s3191::minimum_operations_to_make_binary_array_elements_equal_to_one_i::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1, 0, 0]), 3);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 1]), -1);
}
