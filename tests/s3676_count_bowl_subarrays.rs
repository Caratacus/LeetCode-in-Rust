// Tests for Problem 3676: Count Bowl Subarrays
// Java reference: src/test/java/g3601_3700/s3676_count_bowl_subarrays/SolutionTest.java
use leetcode_in_rust::s3676::count_bowl_subarrays::Solution;
#[test]
fn test_bowl_subarrays() { assert_eq!(Solution::bowl_subarrays(vec![2, 5, 3, 1, 4]), 2i64); }
#[test]
fn test_bowl_subarrays2() { assert_eq!(Solution::bowl_subarrays(vec![5, 1, 2, 3, 4]), 3i64); }
#[test]
fn test_bowl_subarrays3() { assert_eq!(Solution::bowl_subarrays(vec![1000000000, 999999999, 999999998]), 0i64); }
