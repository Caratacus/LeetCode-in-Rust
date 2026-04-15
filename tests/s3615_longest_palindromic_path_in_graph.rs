// Tests for Problem 3615: Longest Palindromic Path in Graph
// Java reference: src/test/java/g3601_3700/s3615_longest_palindromic_path_in_graph/SolutionTest.java
use leetcode_in_rust::s3615::longest_palindromic_path_in_graph::Solution;
#[test]
fn test_max_len() { assert_eq!(Solution::max_len(3, vec![vec![0, 1], vec![1, 2]], "aba".to_string()), 3); }
#[test]
fn test_max_len2() { assert_eq!(Solution::max_len(3, vec![vec![0, 1], vec![0, 2]], "abc".to_string()), 1); }
#[test]
fn test_max_len3() { assert_eq!(Solution::max_len(4, vec![vec![0, 2], vec![0, 3], vec![3, 1]], "bbac".to_string()), 3); }
#[test]
fn test_max_len4() { assert_eq!(Solution::max_len(3, vec![vec![2, 0], vec![2, 1]], "mll".to_string()), 2); }
