// Tests for Problem 1930: Unique Length-3 Palindromic Subsequences
// Java reference: src/test/java/g1901_2000/s1930_unique_length_3_palindromic_subsequences/SolutionTest.java

use leetcode_in_rust::s1930::unique_length_3_palindromic_subsequences::Solution;

#[test]
fn test_count_palindromic_subsequence() {
    assert_eq!(Solution::count_palindromic_subsequence("aabca".to_string()), 3);
}

#[test]
fn test_count_palindromic_subsequence2() {
    assert_eq!(Solution::count_palindromic_subsequence("abc".to_string()), 0);
}

#[test]
fn test_count_palindromic_subsequence3() {
    assert_eq!(Solution::count_palindromic_subsequence("bbcbaba".to_string()), 4);
}
