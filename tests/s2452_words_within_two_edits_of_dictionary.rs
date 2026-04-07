// Tests for Problem 2452: Words Within Two Edits of Dictionary
// Java reference: src/test/java/g2401_2500/s2452_words_within_two_edits_of_dictionary/SolutionTest.java

use leetcode_in_rust::s2452::words_within_two_edits_of_dictionary::Solution;

#[test]
fn test_two_edit_words() {
    assert_eq!(
        Solution::two_edit_words(
            vec!["word".to_string(), "note".to_string(), "ants".to_string(), "wood".to_string()],
            vec!["wood".to_string(), "joke".to_string(), "moat".to_string()]
        ),
        vec!["word".to_string(), "note".to_string(), "wood".to_string()]
    );
}

#[test]
fn test_two_edit_words2() {
    assert_eq!(
        Solution::two_edit_words(vec!["yes".to_string()], vec!["not".to_string()]),
        Vec::<String>::new()
    );
}
