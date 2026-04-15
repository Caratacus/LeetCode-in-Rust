// Tests for Problem 3714: Longest Balanced Substring II
// Java reference: src/test/java/g3701_3800/s3714_longest_balanced_substring_ii/SolutionTest.java
use leetcode_in_rust::s3714::longest_balanced_substring_ii::Solution;
#[test]
fn test_longest_balanced() { assert_eq!(Solution::longest_balanced("abbac".to_string()), 4); }
#[test]
fn test_longest_balanced2() { assert_eq!(Solution::longest_balanced("aabcc".to_string()), 3); }
#[test]
fn test_longest_balanced3() { assert_eq!(Solution::longest_balanced("aba".to_string()), 2); }
