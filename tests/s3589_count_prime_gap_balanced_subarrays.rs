// Tests for Problem 3589: Count Prime Gap Balanced Subarrays
// Java reference: src/test/java/g3501_3600/s3589_count_prime_gap_balanced_subarrays/SolutionTest.java
use leetcode_in_rust::s3589::count_prime_gap_balanced_subarrays::Solution;
#[test] fn test_prime_subarray() { assert_eq!(Solution::prime_subarray(vec![1, 2, 3], 1), 2); }
#[test] fn test_prime_subarray2() { assert_eq!(Solution::prime_subarray(vec![2, 3, 5, 7], 3), 4); }
