// Tests for Problem 3605: Minimum Stability Factor of Array
// Java reference: src/test/java/g3601_3700/s3605_minimum_stability_factor_of_array/SolutionTest.java
use leetcode_in_rust::s3605::minimum_stability_factor_of_array::Solution;
#[test] fn test_min_stable() { assert_eq!(Solution::min_stable(vec![3, 5, 10], 1), 1); }
#[test] fn test_min_stable2() { assert_eq!(Solution::min_stable(vec![2, 6, 8], 2), 1); }
#[test] fn test_min_stable3() { assert_eq!(Solution::min_stable(vec![2, 4, 9, 6], 1), 2); }
