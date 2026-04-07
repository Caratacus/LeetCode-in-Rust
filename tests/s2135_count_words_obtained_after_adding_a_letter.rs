// Tests for Problem 2135: Count Words Obtained After Adding a Letter
// Java reference: src/test/java/g2101_2200/s2135_count_words_obtained_after_adding_a_letter/SolutionTest.java

use leetcode_in_rust::s2135::count_words_obtained_after_adding_a_letter::Solution;

#[test]
fn test_word_count() {
    assert_eq!(
        Solution::word_count(
            vec!["ant".to_string(), "act".to_string(), "tack".to_string()],
            vec!["tack".to_string(), "act".to_string(), "acti".to_string()]
        ),
        2
    );
}

#[test]
fn test_word_count2() {
    assert_eq!(
        Solution::word_count(
            vec!["ab".to_string(), "a".to_string()],
            vec!["abc".to_string(), "abcd".to_string()]
        ),
        1
    );
}
