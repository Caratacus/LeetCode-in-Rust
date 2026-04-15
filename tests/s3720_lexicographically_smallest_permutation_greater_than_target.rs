// Tests for Problem 3720: Lexicographically Smallest Permutation Greater Than Target
// Java reference: src/test/java/g3701_3800/s3720_lexicographically_smallest_permutation_greater_than_target/SolutionTest.java
use leetcode_in_rust::s3720::lexicographically_smallest_permutation_greater_than_target::Solution;
#[test]
fn test_lex_greater_permutation() { assert_eq!(Solution::lex_greater_permutation("abc".to_string(), "bba".to_string()), "bca".to_string()); }
#[test]
fn test_lex_greater_permutation2() { assert_eq!(Solution::lex_greater_permutation("leet".to_string(), "code".to_string()), "eelt".to_string()); }
#[test]
fn test_lex_greater_permutation3() { assert_eq!(Solution::lex_greater_permutation("baba".to_string(), "bbaa".to_string()), "".to_string()); }
