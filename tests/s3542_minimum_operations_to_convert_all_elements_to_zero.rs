// Tests for Problem 3542: Minimum Operations to Convert All Elements to Zero
// Java reference: src/test/java/g3501_3600/s3542_minimum_operations_to_convert_all_elements_to_zero/SolutionTest.java

use leetcode_in_rust::s3542::minimum_operations_to_convert_all_elements_to_zero::Solution;

#[test]
fn test_min_operations() { assert_eq!(Solution::min_operations(vec![0, 2]), 1); }

#[test]
fn test_min_operations2() { assert_eq!(Solution::min_operations(vec![3, 1, 2, 1]), 3); }

#[test]
fn test_min_operations3() { assert_eq!(Solution::min_operations(vec![1, 2, 1, 2, 1, 2]), 4); }
