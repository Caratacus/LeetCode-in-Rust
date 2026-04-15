// Tests for Problem 3677: Count Binary Palindromic Numbers
// Java reference: src/test/java/g3601_3700/s3677_count_binary_palindromic_numbers/SolutionTest.java
use leetcode_in_rust::s3677::count_binary_palindromic_numbers::Solution;
#[test]
fn test_count_binary_palindromes() { assert_eq!(Solution::count_binary_palindromes(9i64), 6); }
#[test]
fn test_count_binary_palindromes3() { assert_eq!(Solution::count_binary_palindromes(0), 1); }
#[test]
fn test_count_binary_palindromes4() { assert_eq!(Solution::count_binary_palindromes(1), 2); }
#[test]
fn test_count_binary_palindromes5() { assert_eq!(Solution::count_binary_palindromes(6), 4); }
#[test]
fn test_count_binary_palindromes6() { assert_eq!(Solution::count_binary_palindromes(9), 6); }
#[test]
fn test_count_binary_palindromes7() { assert_eq!(Solution::count_binary_palindromes(10), 6); }
#[test]
fn test_count_binary_palindromes8() { assert_eq!(Solution::count_binary_palindromes((1i64 << 10) - 1), 63); }
