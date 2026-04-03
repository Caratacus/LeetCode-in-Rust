// Tests for Problem 0809: Expressive Words
// Java reference: src/test/java/g0701_0800/s0809_expressive_words/SolutionTest.java

use leetcode_in_rust::s0809::expressive_words::Solution;

#[test]
fn test_expressive_words() {
    assert_eq!(
        Solution::expressive_words(
            "heeellooo".to_string(),
            vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]
        ),
        1
    );
}

#[test]
fn test_expressive_words2() {
    assert_eq!(
        Solution::expressive_words(
            "zzzzzyyyyy".to_string(),
            vec!["zzyy".to_string(), "zy".to_string(), "zyy".to_string()]
        ),
        3
    );
}
