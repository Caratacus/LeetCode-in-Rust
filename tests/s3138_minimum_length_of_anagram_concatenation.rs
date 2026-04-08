// Tests for Problem 3138: Minimum Length of Anagram Concatenation
// Java reference: src/test/java/g3101_3200/s3138_minimum_length_of_anagram_concatenation/SolutionTest.java

use leetcode_in_rust::s3138::minimum_length_of_anagram_concatenation::Solution;
#[test]
fn test_min_anagram_length() {
    assert_eq!(Solution::min_anagram_length(String::from("abba")), 2);
}
#[test]
fn test_min_anagram_length2() {
    assert_eq!(Solution::min_anagram_length(String::from("cdef")), 4);
}
