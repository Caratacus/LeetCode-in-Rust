// Tests for Problem 3726: Remove Zeros in Decimal Representation
// Java reference: src/test/java/g3701_3800/s3726_remove_zeros_in_decimal_representation/SolutionTest.java
use leetcode_in_rust::s3726::remove_zeros_in_decimal_representation::Solution;
#[test]
fn test_remove_zeros() { assert_eq!(Solution::remove_zeros(1020030i64), 123i64); }
#[test]
fn test_remove_zeros2() { assert_eq!(Solution::remove_zeros(1i64), 1i64); }
