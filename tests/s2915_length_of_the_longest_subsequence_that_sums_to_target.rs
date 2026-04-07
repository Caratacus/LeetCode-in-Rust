// Tests for Problem 2915: Length of the Longest Subsequence That Sums to Target
// Java reference: src/test/java/g2901_3000/s2915_length_of_the_longest_subsequence_that_sums_to_target/SolutionTest.java

use leetcode_in_rust::s2915::length_of_the_longest_subsequence_that_sums_to_target::Solution;

#[test]
fn test_length_of_longest_subsequence() {
    assert_eq!(Solution::length_of_longest_subsequence(vec![1, 2, 3, 4, 5], 9), 3);
}

#[test]
fn test_length_of_longest_subsequence2() {
    assert_eq!(Solution::length_of_longest_subsequence(vec![4, 1, 3, 2, 1, 5], 7), 4);
}

#[test]
fn test_length_of_longest_subsequence3() {
    assert_eq!(Solution::length_of_longest_subsequence(vec![1, 1, 5, 4, 5], 3), -1);
}
