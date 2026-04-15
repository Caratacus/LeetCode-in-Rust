// Tests for Problem 3681: Maximum XOR of Subsequences
// Java reference: src/test/java/g3601_3700/s3681_maximum_xor_of_subsequences/SolutionTest.java
use leetcode_in_rust::s3681::maximum_xor_of_subsequences::Solution;
#[test]
fn test_max_xor_subsequences() { assert_eq!(Solution::max_xor_subsequences(vec![1, 2, 3]), 3); }
#[test]
fn test_max_xor_subsequences2() { assert_eq!(Solution::max_xor_subsequences(vec![5, 2]), 7); }
