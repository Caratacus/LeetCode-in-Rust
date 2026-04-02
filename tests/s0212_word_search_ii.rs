// Tests for Problem 0212: Word Search II
// Java reference: src/test/java/g0201_0300/s0212_word_search_ii/SolutionTest.java

use leetcode_in_rust::s0212::word_search_ii::Solution;

#[test]
fn test_find_words() {
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec!["oath".to_string(), "pea".to_string(), "eat".to_string(), "rain".to_string()];
    let mut result = Solution::find_words(board, words);
    result.sort();
    let mut expected = vec!["oath".to_string(), "eat".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_find_words2() {
    let board = vec![vec!['a', 'b'], vec!['c', 'd']];
    let words = vec!["abcb".to_string()];
    assert_eq!(Solution::find_words(board, words), Vec::<String>::new());
}
