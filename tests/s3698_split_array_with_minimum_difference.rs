// Tests for Problem 3698: Split Array with Minimum Difference
// Java reference: src/test/java/g3601_3700/s3698_split_array_with_minimum_difference/SolutionTest.java
use leetcode_in_rust::s3698::split_array_with_minimum_difference::Solution;
#[test]
fn test_split_array() { assert_eq!(Solution::split_array(vec![1, 3, 2]), 2i64); }
#[test]
fn test_split_array2() { assert_eq!(Solution::split_array(vec![1, 2, 4, 3]), 4i64); }
#[test]
fn test_split_array3() { assert_eq!(Solution::split_array(vec![3, 1, 2]), -1i64); }
#[test]
fn test_split_array4() { assert_eq!(Solution::split_array(vec![1, 2, 3, 4, 5]), 5i64); }
#[test]
fn test_split_array5() { assert_eq!(Solution::split_array(vec![10]), 10i64); }
#[test]
fn test_split_array6() { assert_eq!(Solution::split_array(vec![3, 7]), 4i64); }
#[test]
fn test_split_array7() { assert_eq!(Solution::split_array(vec![1, 2, 2, 1]), 0i64); }
#[test]
fn test_split_array8() { assert_eq!(Solution::split_array(vec![1, 3, 2, 0]), 2i64); }
#[test]
fn test_split_array9() { assert_eq!(Solution::split_array(vec![1, 2, 1, 3]), -1i64); }
#[test]
fn test_split_array10() { assert_eq!(Solution::split_array(vec![2, 4, 3, 1, 2]), -1i64); }
#[test]
fn test_split_array11() { assert_eq!(Solution::split_array(vec![1, 2, 5, 4, 3]), 1i64); }
#[test]
fn test_split_array12() { assert_eq!(Solution::split_array(vec![5, 10, 2, 1]), 8i64); }
#[test]
fn test_split_array13() { assert_eq!(Solution::split_array(vec![2, 3, 1]), 2i64); }
#[test]
fn test_split_array14() { assert_eq!(Solution::split_array(vec![10, 20, 15, 5]), 10i64); }
#[test]
fn test_split_array15() { assert_eq!(Solution::split_array(vec![5, 4, 3, 2, 1]), 5i64); }
#[test]
fn test_split_array16() { assert_eq!(Solution::split_array(vec![3, 3, 3, 2, 1]), -1i64); }
#[test]
fn test_split_array17() { assert_eq!(Solution::split_array(vec![1, 0]), 1i64); }
#[test]
fn test_split_array18() { assert_eq!(Solution::split_array(vec![2, 4, 4, 2]), 0i64); }
#[test]
fn test_split_array19() { assert_eq!(Solution::split_array(vec![1, 10, 9, 8, 7]), 13i64); }
#[test]
fn test_split_array20() { assert_eq!(Solution::split_array(vec![1, 3, 2, 4, 1]), -1i64); }
#[test]
fn test_split_array21() { assert_eq!(Solution::split_array(vec![5, 5, 4, 3]), 7i64); }
#[test]
fn test_split_array22() { assert_eq!(Solution::split_array(vec![100, 200, 10, 5]), 115i64); }
#[test]
fn test_split_array23() { assert_eq!(Solution::split_array(vec![3, 5, 2]), 4i64); }
