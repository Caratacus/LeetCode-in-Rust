// Tests for Problem 3472: Longest Palindromic Subsequence After at Most K Operations
// Java reference: src/test/java/g3401_3500/s3472_longest_palindromic_subsequence_after_at_most_k_operations/SolutionTest.java

use leetcode_in_rust::s3472::longest_palindromic_subsequence_after_at_most_k_operations::Solution;

#[test]
fn test_longest_palindromic_subsequence() {
    assert_eq!(Solution::longest_palindromic_subsequence("abced".to_string(), 2), 3);
}

#[test]
fn test_longest_palindromic_subsequence2() {
    assert_eq!(Solution::longest_palindromic_subsequence("aaazzz".to_string(), 4), 6);
}
