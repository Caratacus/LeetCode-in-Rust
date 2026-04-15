// Tests for Problem 3646: Next Special Palindrome Number
// Java reference: src/test/java/g3601_3700/s3646_next_special_palindrome_number/SolutionTest.java
use leetcode_in_rust::s3646::next_special_palindrome_number::Solution;
#[test]
fn test_special_palindrome() { assert_eq!(Solution::special_palindrome(2i64), 22i64); }
#[test]
fn test_special_palindrome2() { assert_eq!(Solution::special_palindrome(33i64), 212i64); }
