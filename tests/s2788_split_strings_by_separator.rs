// Tests for Problem 2788: Split Strings by Separator
// Java reference: src/test/java/g2701_2800/s2788_split_strings_by_separator/SolutionTest.java

use leetcode_in_rust::s2788::split_strings_by_separator::Solution;

#[test]
fn test_split_words_by_separator() {
    assert_eq!(
        Solution::split_words_by_separator(
            vec!["one.two.three".to_string(), "four.five".to_string(), "six".to_string()],
            '.'
        ),
        vec!["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(), "six".to_string()]
    );
}

#[test]
fn test_split_words_by_separator2() {
    assert_eq!(
        Solution::split_words_by_separator(
            vec!["$easy$".to_string(), "$problem$".to_string()],
            '$'
        ),
        vec!["easy".to_string(), "problem".to_string()]
    );
}
