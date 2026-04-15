// Tests for Problem 3713: Longest Balanced Substring I
// Java reference: src/test/java/g3701_3800/s3713_longest_balanced_substring_i/SolutionTest.java
use leetcode_in_rust::s3713::longest_balanced_substring_i::Solution;
#[test]
fn test_longest_balanced() { assert_eq!(Solution::longest_balanced("abbac".to_string()), 4); }
#[test]
fn test_longest_balanced2() { assert_eq!(Solution::longest_balanced("zzabccy".to_string()), 4); }
#[test]
fn test_longest_balanced3() { assert_eq!(Solution::longest_balanced("aba".to_string()), 2); }
