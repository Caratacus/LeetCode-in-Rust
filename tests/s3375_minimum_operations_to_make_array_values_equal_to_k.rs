// Tests for Problem 3375: Minimum Operations to Make Array Values Equal to K
// Java reference: src/test/java/g3301_3400/s3375_minimum_operations_to_make_array_values_equal_to_k/SolutionTest.java

use leetcode_in_rust::s3375::minimum_operations_to_make_array_values_equal_to_k::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![5, 2, 5, 4, 5], 2), 2);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 1, 2], 2), -1);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![9, 7, 5, 3], 1), 4);
}
