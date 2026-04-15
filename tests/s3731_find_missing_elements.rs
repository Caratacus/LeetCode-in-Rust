// Tests for Problem 3731: Find Missing Elements
// Java reference: src/test/java/g3701_3800/s3731_find_missing_elements/SolutionTest.java
use leetcode_in_rust::s3731::find_missing_elements::Solution;
#[test]
fn test_find_missing_elements() { assert_eq!(Solution::find_missing_elements(vec![1, 4, 2, 5]), vec![3]); }
#[test]
fn test_find_missing_elements2() { assert_eq!(Solution::find_missing_elements(vec![7, 8, 6, 9]), vec![] as Vec<i32>); }
#[test]
fn test_find_missing_elements3() { assert_eq!(Solution::find_missing_elements(vec![5, 1]), vec![2, 3, 4]); }
