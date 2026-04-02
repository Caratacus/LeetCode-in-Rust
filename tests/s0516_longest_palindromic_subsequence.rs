// Tests for Problem 0516: Longest Palindromic Subsequence
// Java reference: src/test/java/g0501_0600/s0516_longest_palindromic_subsequence/SolutionTest.java

use leetcode_in_rust::s0516::longest_palindromic_subsequence::Solution;

#[test]
fn test_longest_palindrome_subseq() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
}

#[test]
fn test_longest_palindrome_subseq2() {
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
}
