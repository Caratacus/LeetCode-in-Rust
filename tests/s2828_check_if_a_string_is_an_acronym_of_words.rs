// Tests for Problem 2828: Check if a String Is an Acronym of Words
// Java reference: src/test/java/g2801_2900/s2828_check_if_a_string_is_an_acronym_of_words/SolutionTest.java

use leetcode_in_rust::s2828::check_if_a_string_is_an_acronym_of_words::Solution;

#[test]
fn test_is_acronym() {
    assert_eq!(Solution::is_acronym(vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()], "abc"), true);
}

#[test]
fn test_is_acronym2() {
    assert_eq!(Solution::is_acronym(vec!["an".to_string(), "apple".to_string()], "a"), false);
}

#[test]
fn test_is_acronym3() {
    assert_eq!(Solution::is_acronym(vec!["never".to_string(), "gonna".to_string(), "give".to_string(), "up".to_string(), "on".to_string(), "you".to_string()], "ngguoy"), true);
}
