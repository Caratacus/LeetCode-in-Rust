// Tests for Problem 3192: Minimum Operations to Make Binary Array Elements Equal to One II
// Java reference: src/test/java/g3101_3200/s3192_minimum_operations_to_make_binary_array_elements_equal_to_one_ii/SolutionTest.java

use leetcode_in_rust::s3192::minimum_operations_to_make_binary_array_elements_equal_to_one_ii::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![0, 1, 1, 0, 1]), 4);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![1, 0, 0, 0]), 1);
}
