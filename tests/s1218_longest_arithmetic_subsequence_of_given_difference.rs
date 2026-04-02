// Tests for Problem 1218: Longest Arithmetic Subsequence of Given Difference
// Java reference: src/test/java/g1201_1300/s1218_longest_arithmetic_subsequence_of_given_difference/SolutionTest.java

use leetcode_in_rust::s1218::longest_arithmetic_subsequence_of_given_difference::Solution;

#[test]
fn test_longest_subsequence() {
    assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
}

#[test]
fn test_longest_subsequence2() {
    assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
}

#[test]
fn test_longest_subsequence3() {
    assert_eq!(Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2), 4);
}
