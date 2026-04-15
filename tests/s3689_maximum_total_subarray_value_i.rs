// Tests for Problem 3689: Maximum Total Subarray Value I
// Java reference: src/test/java/g3601_3700/s3689_maximum_total_subarray_value_i/SolutionTest.java
use leetcode_in_rust::s3689::maximum_total_subarray_value_i::Solution;
#[test]
fn test_max_total_value() { assert_eq!(Solution::max_total_value(vec![1, 3, 2], 2), 4i64); }
#[test]
fn test_max_total_value2() { assert_eq!(Solution::max_total_value(vec![4, 2, 5, 1], 3), 12i64); }
