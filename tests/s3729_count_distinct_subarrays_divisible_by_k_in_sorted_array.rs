// Tests for Problem 3729: Count Distinct Subarrays Divisible by K in Sorted Array
// Java reference: src/test/java/g3701_3800/s3729_count_distinct_subarrays_divisible_by_k_in_sorted_array/SolutionTest.java
use leetcode_in_rust::s3729::count_distinct_subarrays_divisible_by_k_in_sorted_array::Solution;
#[test]
fn test_num_good_subarrays() { assert_eq!(Solution::num_good_subarrays(vec![1, 2, 3], 3), 3i64); }
#[test]
fn test_num_good_subarrays2() { assert_eq!(Solution::num_good_subarrays(vec![2, 2, 2, 2, 2, 2], 6), 2i64); }
