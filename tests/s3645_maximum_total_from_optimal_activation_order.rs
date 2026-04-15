// Tests for Problem 3645: Maximum Total from Optimal Activation Order
// Java reference: src/test/java/g3601_3700/s3645_maximum_total_from_optimal_activation_order/SolutionTest.java
use leetcode_in_rust::s3645::maximum_total_from_optimal_activation_order::Solution;
#[test]
fn test_max_total() { assert_eq!(Solution::max_total(vec![3, 5, 8], vec![2, 1, 3]), 16i64); }
#[test]
fn test_max_total2() { assert_eq!(Solution::max_total(vec![4, 2, 6], vec![1, 1, 1]), 6i64); }
#[test]
fn test_max_total3() { assert_eq!(Solution::max_total(vec![4, 1, 5, 2], vec![3, 3, 2, 3]), 12i64); }
