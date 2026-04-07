// Tests for Problem 2609: Find the Longest Balanced Substring of a Binary String
// Java reference: src/test/java/g2601_2700/s2609_find_the_longest_balanced_substring_of_a_binary_string/SolutionTest.java

use leetcode_in_rust::s2609::find_the_longest_balanced_substring_of_a_binary_string::Solution;

#[test]
fn test_find_the_longest_balanced_substring() {
    assert_eq!(
        Solution::find_the_longest_balanced_substring("01000111".to_string()),
        6
    );
}

#[test]
fn test_find_the_longest_balanced_substring2() {
    assert_eq!(
        Solution::find_the_longest_balanced_substring("00111".to_string()),
        4
    );
}
