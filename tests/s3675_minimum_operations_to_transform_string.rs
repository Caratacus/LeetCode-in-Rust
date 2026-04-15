// Tests for Problem 3675: Minimum Operations to Transform String
// Java reference: src/test/java/g3601_3700/s3675_minimum_operations_to_transform_string/SolutionTest.java
use leetcode_in_rust::s3675::minimum_operations_to_transform_string::Solution;
#[test]
fn test_min_operations() { assert_eq!(Solution::min_operations("yz".to_string()), 2); }
#[test]
fn test_min_operations2() { assert_eq!(Solution::min_operations("a".to_string()), 0); }
