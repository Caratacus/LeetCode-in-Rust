// Tests for Problem 3669: Balanced K Factor Decomposition
// Java reference: src/test/java/g3601_3700/s3669_balanced_k_factor_decomposition/SolutionTest.java
use leetcode_in_rust::s3669::balanced_k_factor_decomposition::Solution;
#[test]
fn test_min_difference() { assert_eq!(Solution::min_difference(100, 2), vec![10, 10]); }
#[test]
fn test_min_difference2() { assert_eq!(Solution::min_difference(44, 3), vec![2, 2, 11]); }
