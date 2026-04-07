// Tests for Problem 2131: Longest Palindrome by Concatenating Two Letter Words
// Java reference: src/test/java/g2101_2200/s2131_longest_palindrome_by_concatenating_two_letter_words/SolutionTest.java

use leetcode_in_rust::s2131::longest_palindrome_by_concatenating_two_letter_words::Solution;

#[test]
fn test_longest_palindrome() {
    assert_eq!(
        Solution::longest_palindrome(vec!["lc".to_string(), "cl".to_string(), "gg".to_string()]),
        6
    );
}

#[test]
fn test_longest_palindrome2() {
    assert_eq!(
        Solution::longest_palindrome(vec![
            "ab".to_string(),
            "ty".to_string(),
            "yt".to_string(),
            "lc".to_string(),
            "cl".to_string(),
            "ab".to_string()
        ]),
        8
    );
}

#[test]
fn test_longest_palindrome3() {
    assert_eq!(
        Solution::longest_palindrome(vec!["cc".to_string(), "ll".to_string(), "xx".to_string()]),
        2
    );
}
