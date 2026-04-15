// Tests for Problem 3692: Majority Frequency Characters
// Java reference: src/test/java/g3601_3700/s3692_majority_frequency_characters/SolutionTest.java
use leetcode_in_rust::s3692::majority_frequency_characters::Solution;
#[test]
fn test_majority_frequency_group() { assert_eq!(Solution::majority_frequency_group("aaabbbccdddde".to_string()), "ab".to_string()); }
#[test]
fn test_majority_frequency_group2() { assert_eq!(Solution::majority_frequency_group("abcd".to_string()), "abcd".to_string()); }
#[test]
fn test_majority_frequency_group3() { assert_eq!(Solution::majority_frequency_group("pfpfgi".to_string()), "fp".to_string()); }
