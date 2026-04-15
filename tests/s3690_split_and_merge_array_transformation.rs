// Tests for Problem 3690: Split and Merge Array Transformation
// Java reference: src/test/java/g3601_3700/s3690_split_and_merge_array_transformation/SolutionTest.java
use leetcode_in_rust::s3690::split_and_merge_array_transformation::Solution;
#[test]
fn test_min_split_merge() { assert_eq!(Solution::min_split_merge(vec![3, 1, 2], vec![1, 2, 3]), 1); }
#[test]
fn test_min_split_merge2() { assert_eq!(Solution::min_split_merge(vec![1, 1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1, 1]), 3); }
