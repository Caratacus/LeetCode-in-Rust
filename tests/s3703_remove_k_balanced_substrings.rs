// Tests for Problem 3703: Remove K Balanced Substrings
// Java reference: src/test/java/g3701_3800/s3703_remove_k_balanced_substrings/SolutionTest.java
use leetcode_in_rust::s3703::remove_k_balanced_substrings::Solution;
#[test]
fn test_remove_substring() { assert_eq!(Solution::remove_substring("(())".to_string(), 1), "".to_string()); }
#[test]
fn test_remove_substring2() { assert_eq!(Solution::remove_substring("(()(".to_string(), 1), "((".to_string()); }
#[test]
fn test_remove_substring3() { assert_eq!(Solution::remove_substring("((()))()()()".to_string(), 3), "()()()".to_string()); }
