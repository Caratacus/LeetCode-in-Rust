// Tests for Problem 3504: Longest Palindrome After Substring Concatenation II
// Java reference: src/test/java/g3501_3600/s3504_longest_palindrome_after_substring_concatenation_ii/SolutionTest.java

use leetcode_in_rust::s3504::longest_palindrome_after_substring_concatenation_ii::Solution;

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

#[test]
fn test_longest_palindrome5() {
    assert_eq!(Solution::longest_palindrome("xxz".to_string(), "z".to_string()), 2);
}
