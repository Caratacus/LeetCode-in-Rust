// Tests for Problem 3674: Minimum Operations to Equalize Array
// Java reference: src/test/java/g3601_3700/s3674_minimum_operations_to_equalize_array/SolutionTest.java
use leetcode_in_rust::s3674::minimum_operations_to_equalize_array::Solution;
#[test]
fn test_min_operations() { assert_eq!(Solution::min_operations(vec![1, 2]), 1); }
#[test]
fn test_min_operations2() { assert_eq!(Solution::min_operations(vec![5, 5, 5]), 0); }
