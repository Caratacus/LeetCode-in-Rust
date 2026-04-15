// Tests for Problem 3704: Count No Zero Pairs That Sum to N
// Java reference: src/test/java/g3701_3800/s3704_count_no_zero_pairs_that_sum_to_n/SolutionTest.java
use leetcode_in_rust::s3704::count_no_zero_pairs_that_sum_to_n::Solution;
#[test]
fn test_count_no_zero_pairs() { assert_eq!(Solution::count_no_zero_pairs(2i64), 1i64); }
#[test]
fn test_count_no_zero_pairs2() { assert_eq!(Solution::count_no_zero_pairs(3i64), 2i64); }
#[test]
fn test_count_no_zero_pairs3() { assert_eq!(Solution::count_no_zero_pairs(11i64), 8i64); }
