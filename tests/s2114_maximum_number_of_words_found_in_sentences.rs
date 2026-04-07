// Tests for Problem 2114: Maximum Number of Words Found in Sentences
// Java reference: src/test/java/g2101_2200/s2114_maximum_number_of_words_found_in_sentences/SolutionTest.java

use leetcode_in_rust::s2114::maximum_number_of_words_found_in_sentences::Solution;

#[test]
fn test_most_words_found() {
    assert_eq!(
        Solution::most_words_found(vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string()
        ]),
        6
    );
}

#[test]
fn test_most_words_found2() {
    assert_eq!(
        Solution::most_words_found(vec![
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string()
        ]),
        3
    );
}
