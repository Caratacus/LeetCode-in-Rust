// Tests for Problem 1143: Longest Common Subsequence
// Java reference: src/test/java/g1101_1200/s1143_longest_common_subsequence/SolutionTest.java

use leetcode_in_rust::s1143::longest_common_subsequence::Solution;

#[test]
fn test_longest_common_subsequence() {
    let result = Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string());
    assert_eq!(result, 3);
}

#[test]
fn test_longest_common_subsequence2() {
    let result = Solution::longest_common_subsequence("abc".to_string(), "abc".to_string());
    assert_eq!(result, 3);
}

#[test]
fn test_longest_common_subsequence3() {
    let result = Solution::longest_common_subsequence("abc".to_string(), "def".to_string());
    assert_eq!(result, 0);
}
