// Tests for Problem 3708: Longest Fibonacci Subarray
// Java reference: src/test/java/g3701_3800/s3708_longest_fibonacci_subarray/SolutionTest.java
use leetcode_in_rust::s3708::longest_fibonacci_subarray::Solution;
#[test]
fn test_longest_subarray() { assert_eq!(Solution::longest_subarray(vec![1, 1, 1, 1, 2, 3, 5, 1]), 5); }
#[test]
fn test_longest_subarray2() { assert_eq!(Solution::longest_subarray(vec![5, 2, 7, 9, 16]), 5); }
#[test]
fn test_longest_subarray3() { assert_eq!(Solution::longest_subarray(vec![1000000000, 1000000000, 1000000000]), 2); }
