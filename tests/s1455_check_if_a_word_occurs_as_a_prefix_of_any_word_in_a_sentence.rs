// Tests for Problem 1455: Check If a Word Occurs As a Prefix of Any Word in a Sentence
// Java reference: src/test/java/g1401_1500/s1455_check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence/SolutionTest.java

use leetcode_in_rust::s1455::check_if_a_word_occurs_as_a_prefix_of_any_word_in_a_sentence::Solution;

#[test]
fn test_is_prefix_of_word() {
    assert_eq!(Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()), 4);
}

#[test]
fn test_is_prefix_of_word2() {
    assert_eq!(Solution::is_prefix_of_word("this problem is an easy problem".to_string(), "pro".to_string()), 2);
}

#[test]
fn test_is_prefix_of_word3() {
    assert_eq!(Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()), -1);
}
