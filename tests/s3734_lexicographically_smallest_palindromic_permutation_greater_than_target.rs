// Tests for Problem 3734: Lexicographically Smallest Palindromic Permutation Greater Than Target
// Java reference: src/test/java/g3701_3800/s3734_lexicographically_smallest_palindromic_permutation_greater_than_target/SolutionTest.java
use leetcode_in_rust::s3734::lexicographically_smallest_palindromic_permutation_greater_than_target::Solution;
#[test]
fn test_lex_palindromic_permutation() { assert_eq!(Solution::lex_palindromic_permutation("baba".to_string(), "abba".to_string()), "baab".to_string()); }
#[test]
fn test_lex_palindromic_permutation2() { assert_eq!(Solution::lex_palindromic_permutation("baba".to_string(), "bbaa".to_string()), "".to_string()); }
#[test]
fn test_lex_palindromic_permutation3() { assert_eq!(Solution::lex_palindromic_permutation("abc".to_string(), "abb".to_string()), "".to_string()); }
#[test]
fn test_lex_palindromic_permutation4() { assert_eq!(Solution::lex_palindromic_permutation("aac".to_string(), "abb".to_string()), "aca".to_string()); }
