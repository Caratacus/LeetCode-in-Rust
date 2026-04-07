// Tests for Problem 2484: Count Palindromic Subsequences
// Java reference: src/test/java/g2401_2500/s2484_count_palindromic_subsequences/SolutionTest.java

use leetcode_in_rust::s2484::count_palindromic_subsequences::Solution;

#[test]
fn test_count_palindromes() {
    assert_eq!(Solution::count_palindromes("103301".to_string()), 2);
}

#[test]
fn test_count_palindromes2() {
    assert_eq!(Solution::count_palindromes("0000000".to_string()), 21);
}

#[test]
fn test_count_palindromes3() {
    assert_eq!(Solution::count_palindromes("9999900000".to_string()), 2);
}
