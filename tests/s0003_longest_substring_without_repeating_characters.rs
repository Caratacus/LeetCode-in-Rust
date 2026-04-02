// Tests for Problem 0003: Longest Substring Without Repeating Characters
// Java reference: src/test/java/g0001_0100/s0003_longest_substring_without_repeating_characters/SolutionTest.java

use leetcode_in_rust::s0003::longest_substring_without_repeating_characters::Solution;

#[test]
fn test_length_of_longest_substring() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
}

#[test]
fn test_length_of_longest_substring2() {
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
}

#[test]
fn test_length_of_longest_substring3() {
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
}
