// Tests for Problem 3644: Maximum K to Sort a Permutation
// Java reference: src/test/java/g3601_3700/s3644_maximum_k_to_sort_a_permutation/SolutionTest.java
use leetcode_in_rust::s3644::maximum_k_to_sort_a_permutation::Solution;
#[test]
fn test_sort_permutation() { assert_eq!(Solution::sort_permutation(vec![0, 3, 2, 1]), 1); }
#[test]
fn test_sort_permutation2() { assert_eq!(Solution::sort_permutation(vec![0, 1, 3, 2]), 2); }
#[test]
fn test_sort_permutation3() { assert_eq!(Solution::sort_permutation(vec![3, 2, 1, 0]), 0); }
#[test]
fn test_sort_permutation4() { assert_eq!(Solution::sort_permutation(vec![0, 1]), 0); }
