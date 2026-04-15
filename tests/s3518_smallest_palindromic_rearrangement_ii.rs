// Tests for Problem 3518: Smallest Palindromic Rearrangement II
// Java reference: src/test/java/g3501_3600/s3518_smallest_palindromic_rearrangement_ii/SolutionTest.java

use leetcode_in_rust::s3518::smallest_palindromic_rearrangement_ii::Solution;

#[test]
fn test_smallest_palindrome() {
    assert_eq!(Solution::smallest_palindrome("abba".to_string(), 2), "baab".to_string());
}

#[test]
fn test_smallest_palindrome2() {
    assert_eq!(Solution::smallest_palindrome("aa".to_string(), 2), "".to_string());
}

#[test]
fn test_smallest_palindrome3() {
    assert_eq!(Solution::smallest_palindrome("bacab".to_string(), 1), "abcba".to_string());
}
