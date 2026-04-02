// Tests for Problem 0524: Longest Word in Dictionary through Deleting
// Java reference: src/test/java/g0501_0600/s0524_longest_word_in_dictionary_through_deleting/SolutionTest.java

use leetcode_in_rust::s0524::longest_word_in_dictionary_through_deleting::Solution;

#[test]
fn test_find_longest_word() {
    assert_eq!(
        Solution::find_longest_word("abpcplea".to_string(), vec!["ale".to_string(), "apple".to_string(), "monkey".to_string(), "plea".to_string()]),
        "apple"
    );
}

#[test]
fn test_find_longest_word2() {
    assert_eq!(
        Solution::find_longest_word("abpcplea".to_string(), vec!["a".to_string(), "b".to_string(), "c".to_string()]),
        "a"
    );
}
