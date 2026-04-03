// Tests for Problem 0720: Longest Word in Dictionary
// Java reference: src/test/java/g0701_0800/s0720_longest_word_in_dictionary/SolutionTest.java

use leetcode_in_rust::s0720::longest_word_in_dictionary::Solution;

#[test]
fn test_longest_word() {
    let words = vec![
        "w".to_string(),
        "wo".to_string(),
        "wor".to_string(),
        "worl".to_string(),
        "world".to_string(),
    ];
    let result = Solution::longest_word(words);
    assert_eq!(result, "world");
}

#[test]
fn test_longest_word2() {
    let words = vec![
        "a".to_string(),
        "banana".to_string(),
        "app".to_string(),
        "appl".to_string(),
        "ap".to_string(),
        "apply".to_string(),
        "apple".to_string(),
    ];
    let result = Solution::longest_word(words);
    assert_eq!(result, "apple");
}
