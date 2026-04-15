// Tests for Problem 3719: Longest Balanced Subarray I
// Java reference: src/test/java/g3701_3800/s3719_longest_balanced_subarray_i/SolutionTest.java
use leetcode_in_rust::s3719::longest_balanced_subarray_i::Solution;
#[test]
fn test_longest_balanced() { assert_eq!(Solution::longest_balanced(vec![2, 5, 4, 3]), 4); }
#[test]
fn test_longest_balanced2() { assert_eq!(Solution::longest_balanced(vec![3, 2, 2, 5, 4]), 5); }
#[test]
fn test_longest_balanced3() { assert_eq!(Solution::longest_balanced(vec![1, 2, 3, 2]), 3); }
