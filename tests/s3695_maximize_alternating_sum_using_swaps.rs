// Tests for Problem 3695: Maximize Alternating Sum Using Swaps
// Java reference: src/test/java/g3601_3700/s3695_maximize_alternating_sum_using_swaps/SolutionTest.java
use leetcode_in_rust::s3695::maximize_alternating_sum_using_swaps::Solution;
#[test]
fn test_max_alternating_sum() { assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3], vec![vec![0, 2], vec![1, 2]]), 4i64); }
#[test]
fn test_max_alternating_sum2() { assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3], vec![vec![1, 2]]), 2i64); }
#[test]
fn test_max_alternating_sum3() { assert_eq!(Solution::max_alternating_sum(vec![1, 1000000000, 1, 1000000000, 1, 1000000000], vec![]), -2999999997i64); }
