// Tests for Problem 0395: Longest Substring with At Least K Repeating Characters
// Java reference: src/test/java/g0301_0400/s0395_longest_substring_with_at_least_k_repeating_characters/SolutionTest.java

use leetcode_in_rust::s0395::longest_substring_with_at_least_k_repeating_characters::Solution;

#[test]
fn test_longest_substring() {
    assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
}

#[test]
fn test_longest_substring2() {
    assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
}
