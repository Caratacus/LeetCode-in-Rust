// Tests for Problem 3724: Minimum Operations to Transform Array
// Java reference: src/test/java/g3701_3800/s3724_minimum_operations_to_transform_array/SolutionTest.java
use leetcode_in_rust::s3724::minimum_operations_to_transform_array::Solution;
#[test]
fn test_min_operations() { assert_eq!(Solution::min_operations(vec![2, 8], vec![1, 7, 3]), 4i64); }
#[test]
fn test_min_operations2() { assert_eq!(Solution::min_operations(vec![1, 3, 6], vec![2, 4, 5, 3]), 4i64); }
#[test]
fn test_min_operations3() { assert_eq!(Solution::min_operations(vec![2], vec![3, 4]), 3i64); }
