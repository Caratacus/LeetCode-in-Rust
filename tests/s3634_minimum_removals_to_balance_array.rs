// Tests for Problem 3634: Minimum Removals to Balance Array
// Java reference: src/test/java/g3601_3700/s3634_minimum_removals_to_balance_array/SolutionTest.java
use leetcode_in_rust::s3634::minimum_removals_to_balance_array::Solution;
#[test]
fn test_min_removal() { assert_eq!(Solution::min_removal(vec![2, 1, 5], 2), 1); }
#[test]
fn test_min_removal2() { assert_eq!(Solution::min_removal(vec![1, 6, 2, 9], 3), 2); }
#[test]
fn test_min_removal3() { assert_eq!(Solution::min_removal(vec![4, 6], 2), 0); }
