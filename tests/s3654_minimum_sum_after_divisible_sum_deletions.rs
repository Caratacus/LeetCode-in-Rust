// Tests for Problem 3654: Minimum Sum After Divisible Sum Deletions
// Java reference: src/test/java/g3601_3700/s3654_minimum_sum_after_divisible_sum_deletions/SolutionTest.java
use leetcode_in_rust::s3654::minimum_sum_after_divisible_sum_deletions::Solution;
#[test]
fn test_min_array_sum() { assert_eq!(Solution::min_array_sum(vec![1, 1, 1], 2), 1i64); }
#[test]
fn test_min_array_sum2() { assert_eq!(Solution::min_array_sum(vec![3, 1, 4, 1, 5], 3), 5i64); }
