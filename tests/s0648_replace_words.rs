// Tests for Problem 0648: Replace Words
// Java reference: src/test/java/g0601_0700/s0648_replace_words/SolutionTest.java

use leetcode_in_rust::s0648::replace_words::Solution;

#[test]
fn test_replace_words() {
    let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
    let sentence = "the cattle was rattled by the battery".to_string();
    assert_eq!(
        Solution::replace_words(dictionary, sentence),
        "the cat was rat by the bat"
    );
}

#[test]
fn test_replace_words2() {
    let dictionary = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let sentence = "aadsfasf absbs bbab cadsfafs".to_string();
    assert_eq!(Solution::replace_words(dictionary, sentence), "a a b c");
}
