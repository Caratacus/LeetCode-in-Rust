// Tests for Problem 3637: Trionic Array I
// Java reference: src/test/java/g3601_3700/s3637_trionic_array_i/SolutionTest.java
use leetcode_in_rust::s3637::trionic_array_i::Solution;
#[test]
fn test_is_trionic() { assert_eq!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]), true); }
#[test]
fn test_is_trionic2() { assert_eq!(Solution::is_trionic(vec![2, 1, 3]), false); }
#[test]
fn test_is_trionic3() { assert_eq!(Solution::is_trionic(vec![3, 7, 1]), false); }
