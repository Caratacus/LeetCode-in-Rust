// Tests for Problem 3666: Minimum Operations to Equalize Binary String
// Java reference: src/test/java/g3601_3700/s3666_minimum_operations_to_equalize_binary_string/SolutionTest.java
use leetcode_in_rust::s3666::minimum_operations_to_equalize_binary_string::Solution;
#[test]
fn test_min_operations() { assert_eq!(Solution::min_operations("110".to_string(), 1), 1); }
#[test]
fn test_min_operations2() { assert_eq!(Solution::min_operations("0101".to_string(), 3), 2); }
#[test]
fn test_min_operations3() { assert_eq!(Solution::min_operations("101".to_string(), 2), -1); }
#[test]
fn test_min_operations4() { assert_eq!(Solution::min_operations("111111".to_string(), 3), 0); }
#[test]
fn test_min_operations5() { assert_eq!(Solution::min_operations("000000".to_string(), 6), 1); }
#[test]
fn test_min_operations6() { assert_eq!(Solution::min_operations("000111".to_string(), 6), -1); }
#[test]
fn test_min_operations7() { assert_eq!(Solution::min_operations("0011".to_string(), 3), 2); }
#[test]
fn test_min_operations8() { assert_eq!(Solution::min_operations("000011".to_string(), 4), 1); }
#[test]
fn test_min_operations9() { assert_eq!(Solution::min_operations("000111".to_string(), 2), -1); }
#[test]
fn test_min_operations10() { assert_eq!(Solution::min_operations("001100".to_string(), 4), 1); }
#[test]
fn test_min_operations11() { assert_eq!(Solution::min_operations("000100".to_string(), 3), 3); }
#[test]
fn test_min_operations12() { assert_eq!(Solution::min_operations("111111".to_string(), 4), 0); }
#[test]
fn test_min_operations13() { assert_eq!(Solution::min_operations("001001".to_string(), 4), 1); }
