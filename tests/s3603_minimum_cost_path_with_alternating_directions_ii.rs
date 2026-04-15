// Tests for Problem 3603: Minimum Cost Path with Alternating Directions II
// Java reference: src/test/java/g3601_3700/s3603_minimum_cost_path_with_alternating_directions_ii/SolutionTest.java
use leetcode_in_rust::s3603::minimum_cost_path_with_alternating_directions_ii::Solution;
#[test] fn test_min_cost() { assert_eq!(Solution::min_cost(1, 2, vec![vec![1, 2]]), 3i64); }
#[test] fn test_min_cost2() { assert_eq!(Solution::min_cost(2, 2, vec![vec![3, 5], vec![2, 4]]), 9i64); }
#[test] fn test_min_cost3() { assert_eq!(Solution::min_cost(2, 3, vec![vec![6, 1, 4], vec![3, 2, 5]]), 16i64); }
