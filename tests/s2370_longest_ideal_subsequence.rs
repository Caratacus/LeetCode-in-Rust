// Tests for Problem 2370: Longest Ideal Subsequence
// Java reference: src/test/java/g2301_2400/s2370_longest_ideal_subsequence/SolutionTest.java

use leetcode_in_rust::s2370::longest_ideal_subsequence::Solution;

#[test]
fn test_longest_ideal_string() {
    assert_eq!(Solution::longest_ideal_string("acfgbd".to_string(), 2), 4);
}

#[test]
fn test_longest_ideal_string2() {
    assert_eq!(Solution::longest_ideal_string("abcd".to_string(), 3), 4);
}
