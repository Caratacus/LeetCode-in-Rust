// Tests for Problem 3229: Minimum Operations to Make Array Equal to Target
// Java reference: src/test/java/g3201_3300/s3229_minimum_operations_to_make_array_equal_to_target/SolutionTest.java

use leetcode_in_rust::s3229::minimum_operations_to_make_array_equal_to_target::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![3, 5, 1, 2], vec![4, 6, 2, 4]), 2);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![1, 3, 2], vec![2, 1, 4]), 5);
}
