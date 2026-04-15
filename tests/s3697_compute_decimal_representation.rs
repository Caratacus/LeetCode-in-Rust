// Tests for Problem 3697: Compute Decimal Representation
// Java reference: src/test/java/g3601_3700/s3697_compute_decimal_representation/SolutionTest.java
use leetcode_in_rust::s3697::compute_decimal_representation::Solution;
#[test]
fn test_decimal_representation() { assert_eq!(Solution::decimal_representation(537), vec![500, 30, 7]); }
#[test]
fn test_decimal_representation2() { assert_eq!(Solution::decimal_representation(102), vec![100, 2]); }
#[test]
fn test_decimal_representation3() { assert_eq!(Solution::decimal_representation(6), vec![6]); }
