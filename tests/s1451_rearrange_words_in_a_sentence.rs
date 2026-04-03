// Tests for Problem 1451: Rearrange Words in a Sentence
// Java reference: src/test/java/g1401_1500/s1451_rearrange_words_in_a_sentence/SolutionTest.java

use leetcode_in_rust::s1451::rearrange_words_in_a_sentence::Solution;

#[test]
fn test_arrange_words() {
    assert_eq!(Solution::arrange_words("Leetcode is cool".to_string()), "Is cool leetcode");
}

#[test]
fn test_arrange_words2() {
    assert_eq!(Solution::arrange_words("Keep calm and code on".to_string()), "On and keep calm code");
}

#[test]
fn test_arrange_words3() {
    assert_eq!(Solution::arrange_words("To be or not to be".to_string()), "To be or to be not");
}
