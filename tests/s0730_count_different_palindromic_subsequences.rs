// Tests for Problem 0730: Count Different Palindromic Subsequences
// Java reference: src/test/java/g0701_0800/s0730_count_different_palindromic_subsequences/SolutionTest.java

use leetcode_in_rust::s0730::count_different_palindromic_subsequences::Solution;

#[test]
fn test_count_palindromic_subsequences() {
    assert_eq!(Solution::count_palindromic_subsequences("bccb".to_string()), 6);
}

#[test]
fn test_count_palindromic_subsequences2() {
    assert_eq!(
        Solution::count_palindromic_subsequences(
            "abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba".to_string()
        ),
        104860361
    );
}
