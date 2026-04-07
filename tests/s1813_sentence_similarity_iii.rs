// Tests for Problem 1813: Sentence Similarity III
// Java reference: src/test/java/g1801_1900/s1813_sentence_similarity_iii/SolutionTest.java

use leetcode_in_rust::s1813::sentence_similarity_iii::Solution;

#[test]
fn test_are_sentences_similar() {
    assert_eq!(
        Solution::are_sentences_similar("My name is Haley".to_string(), "My Haley".to_string()),
        true
    );
}

#[test]
fn test_are_sentences_similar2() {
    assert_eq!(
        Solution::are_sentences_similar("of".to_string(), "A lot of words".to_string()),
        false
    );
}

#[test]
fn test_are_sentences_similar3() {
    assert_eq!(
        Solution::are_sentences_similar("Eating right now".to_string(), "Eating".to_string()),
        true
    );
}
