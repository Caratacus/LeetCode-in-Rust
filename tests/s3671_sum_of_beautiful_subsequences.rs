// Tests for Problem 3671: Sum of Beautiful Subsequences
// Java reference: src/test/java/g3601_3700/s3671_sum_of_beautiful_subsequences/SolutionTest.java
use leetcode_in_rust::s3671::sum_of_beautiful_subsequences::Solution;
#[test]
fn test_total_beauty() { assert_eq!(Solution::total_beauty(vec![1, 2, 3]), 10); }
#[test]
fn test_total_beauty2() { assert_eq!(Solution::total_beauty(vec![4, 6]), 12); }
