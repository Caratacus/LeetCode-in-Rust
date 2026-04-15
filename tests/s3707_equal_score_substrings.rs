// Tests for Problem 3707: Equal Score Substrings
// Java reference: src/test/java/g3701_3800/s3707_equal_score_substrings/SolutionTest.java
use leetcode_in_rust::s3707::equal_score_substrings::Solution;
#[test]
fn test_score_balance() { assert_eq!(Solution::score_balance("adcb".to_string()), true); }
#[test]
fn test_score_balance2() { assert_eq!(Solution::score_balance("bace".to_string()), false); }
