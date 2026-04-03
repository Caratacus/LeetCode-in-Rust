// Tests for Problem 0884: Uncommon Words from Two Sentences
// Java reference: src/test/java/g0801_0900/s0884_uncommon_words_from_two_sentences/SolutionTest.java

use leetcode_in_rust::s0884::uncommon_words_from_two_sentences::Solution;

#[test]
fn test_uncommon_from_sentences() {
    let mut result = Solution::uncommon_from_sentences(
        "this apple is sweet".to_string(),
        "this apple is sour".to_string()
    );
    result.sort();
    assert_eq!(result, vec!["sour", "sweet"]);
}

#[test]
fn test_uncommon_from_sentences2() {
    let mut result = Solution::uncommon_from_sentences(
        "apple apple".to_string(),
        "banana".to_string()
    );
    result.sort();
    assert_eq!(result, vec!["banana"]);
}
