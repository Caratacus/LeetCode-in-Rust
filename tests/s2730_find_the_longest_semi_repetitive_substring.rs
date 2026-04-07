// Tests for Problem 2730: Find the Longest Semi-Repetitive Substring
// Java reference: src/test/java/g2701_2800/s2730_find_the_longest_semi_repetitive_substring/SolutionTest.java

use leetcode_in_rust::s2730::find_the_longest_semi_repetitive_substring::Solution;

#[test]
fn test_longest_semi_repetitive_substring() {
    assert_eq!(Solution::longest_semi_repetitive_substring("52233".to_string()), 4);
}

#[test]
fn test_longest_semi_repetitive_substring2() {
    assert_eq!(Solution::longest_semi_repetitive_substring("5494".to_string()), 4);
}

#[test]
fn test_longest_semi_repetitive_substring3() {
    assert_eq!(Solution::longest_semi_repetitive_substring("1111111".to_string()), 2);
}
