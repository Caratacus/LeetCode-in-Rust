// Tests for Problem 3701: Compute Alternating Sum
// Java reference: src/test/java/g3701_3800/s3701_compute_alternating_sum/SolutionTest.java
use leetcode_in_rust::s3701::compute_alternating_sum::Solution;
#[test]
fn test_alternating_sum() { assert_eq!(Solution::alternating_sum(vec![1, 3, 5, 7]), -4); }
#[test]
fn test_alternating_sum2() { assert_eq!(Solution::alternating_sum(vec![100]), 100); }
