// Tests for Problem 1332: Remove Palindromic Subsequences
// Java reference: src/test/java/g1301_1400/s1332_remove_palindromic_subsequences/SolutionTest.java

use leetcode_in_rust::s1332::remove_palindromic_subsequences::Solution;

#[test]
fn test_remove_palindrome_sub() {
    assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
}

#[test]
fn test_remove_palindrome_sub2() {
    assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
}

#[test]
fn test_remove_palindrome_sub3() {
    assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
}
