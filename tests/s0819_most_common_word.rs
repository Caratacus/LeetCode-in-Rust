// Tests for Problem 0819: Most Common Word
// Java reference: src/test/java/g0701_0800/s0819_most_common_word/solutionTest.java

use leetcode_in_rust::s0819::most_common_word::Solution;

#[test]
fn test_most_common_word() {
    assert_eq!(
        Solution::most_common_word(
            "Bob hit a ball, the hit BALL flew far after it was hit.".to_string(),
            vec!["hit".to_string()]
        ),
        "ball".to_string()
    );
}

#[test]
fn test_most_common_word2() {
    assert_eq!(
        Solution::most_common_word("a.".to_string(), vec![]),
        "a".to_string()
    );
}
