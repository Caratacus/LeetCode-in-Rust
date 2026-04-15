// Tests for Problem 3640: Trionic Array II
// Java reference: src/test/java/g3601_3700/s3640_trionic_array_ii/SolutionTest.java
use leetcode_in_rust::s3640::trionic_array_ii::Solution;
#[test]
fn test_max_sum_trionic() { assert_eq!(Solution::max_sum_trionic(vec![0, -2, -1, -3, 0, 2, -1]), -4i64); }
#[test]
fn test_max_sum_trionic2() { assert_eq!(Solution::max_sum_trionic(vec![1, 4, 2, 7]), 14i64); }
