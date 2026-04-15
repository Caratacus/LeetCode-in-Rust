// Tests for Problem 3727: Maximum Alternating Sum of Squares
// Java reference: src/test/java/g3701_3800/s3727_maximum_alternating_sum_of_squares/SolutionTest.java
use leetcode_in_rust::s3727::maximum_alternating_sum_of_squares::Solution;
#[test]
fn test_max_alternating_sum() { assert_eq!(Solution::max_alternating_sum(vec![1, 2, 3]), 12i64); }
#[test]
fn test_max_alternating_sum2() { assert_eq!(Solution::max_alternating_sum(vec![1, -1, 2, -2, 3, -3]), 16i64); }
