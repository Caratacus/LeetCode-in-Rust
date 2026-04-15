// Tests for Problem 3691: Maximum Total Subarray Value II
// Java reference: src/test/java/g3601_3700/s3691_maximum_total_subarray_value_ii/SolutionTest.java
use leetcode_in_rust::s3691::maximum_total_subarray_value_ii::Solution;
#[test]
fn test_max_total_value() { assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4i64); }
#[test]
fn test_max_total_value2() { assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12i64); }
