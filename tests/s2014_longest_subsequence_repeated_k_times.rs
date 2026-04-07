// Tests for Problem 2014: Longest Subsequence Repeated k Times
// Java reference: src/test/java/g2001_2100/s2014_longest_subsequence_repeated_k_times/SolutionTest.java

use leetcode_in_rust::s2014::longest_subsequence_repeated_k_times::Solution;

#[test]
fn test_longest_subsequence_repeated_k() {
    assert_eq!(
        Solution::longest_subsequence_repeated_k(String::from("letsleetcode"), 2),
        String::from("let")
    );
}

#[test]
fn test_longest_subsequence_repeated_k2() {
    assert_eq!(
        Solution::longest_subsequence_repeated_k(String::from("bb"), 2),
        String::from("b")
    );
}

#[test]
fn test_longest_subsequence_repeated_k3() {
    assert_eq!(
        Solution::longest_subsequence_repeated_k(String::from("ab"), 2),
        String::from("")
    );
}
