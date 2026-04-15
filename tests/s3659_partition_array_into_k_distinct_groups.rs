// Tests for Problem 3659: Partition Array into K Distinct Groups
// Java reference: src/test/java/g3601_3700/s3659_partition_array_into_k_distinct_groups/SolutionTest.java
use leetcode_in_rust::s3659::partition_array_into_k_distinct_groups::Solution;
#[test]
fn test_partition_array() { assert_eq!(Solution::partition_array(vec![1, 2, 3, 4], 2), true); }
#[test]
fn test_partition_array2() { assert_eq!(Solution::partition_array(vec![3, 5, 2, 2], 2), true); }
#[test]
fn test_partition_array3() { assert_eq!(Solution::partition_array(vec![1, 5, 2, 3], 3), false); }
#[test]
fn test_partition_array4() { assert_eq!(Solution::partition_array(vec![1, 2, 3, 4, 5], 2), false); }
#[test]
fn test_partition_array5() { assert_eq!(Solution::partition_array(vec![1, 2, 1, 2], 2), true); }
#[test]
fn test_partition_array6() { assert_eq!(Solution::partition_array(vec![1, 1, 1, 2], 2), false); }
#[test]
fn test_partition_array7() { assert_eq!(Solution::partition_array(vec![7], 1), true); }
#[test]
fn test_partition_array8() { assert_eq!(Solution::partition_array(vec![5, 5, 5, 5, 5, 5], 3), false); }
