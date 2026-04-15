// Tests for Problem 3503: Longest Palindrome After Substring Concatenation I
// Java reference: src/test/java/g3501_3600/s3503_longest_palindrome_after_substring_concatenation_i/SolutionTest.java

use leetcode_in_rust::s3503::longest_palindrome_after_substring_concatenation_i::Solution;

#[test]
fn test_longest_palindrome() {
    assert_eq!(Solution::longest_palindrome("a".to_string(), "a".to_string()), 2);
}

#[test]
fn test_longest_palindrome2() {
    assert_eq!(Solution::longest_palindrome("abc".to_string(), "def".to_string()), 1);
}

#[test]
fn test_longest_palindrome3() {
    assert_eq!(Solution::longest_palindrome("b".to_string(), "aaaa".to_string()), 4);
}

#[test]
fn test_longest_palindrome4() {
    assert_eq!(Solution::longest_palindrome("abcde".to_string(), "ecdba".to_string()), 5);
}
