// Tests for Problem 3627: Maximum Median Sum of Subsequences of Size 3
// Java reference: src/test/java/g3601_3700/s3627_maximum_median_sum_of_subsequences_of_size_3/SolutionTest.java
use leetcode_in_rust::s3627::maximum_median_sum_of_subsequences_of_size_3::Solution;
#[test]
fn test_maximum_median_sum() { assert_eq!(Solution::maximum_median_sum(vec![2, 1, 3, 2, 1, 3]), 5i64); }
#[test]
fn test_maximum_median_sum2() { assert_eq!(Solution::maximum_median_sum(vec![1, 1, 10, 10, 10, 10]), 20i64); }
