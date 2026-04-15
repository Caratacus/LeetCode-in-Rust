// Tests for Problem 3721: Longest Balanced Subarray II
// Java reference: src/test/java/g3701_3800/s3721_longest_balanced_subarray_ii/SolutionTest.java
use leetcode_in_rust::s3721::longest_balanced_subarray_ii::Solution;
#[test]
fn test_longest_balanced() { assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4); }
#[test]
fn test_longest_balanced2() { assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5); }
#[test]
fn test_longest_balanced3() { assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3); }
