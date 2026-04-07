// Tests for Problem 1880: Check if Word Equals Summation of Two Words
// Java reference: src/test/java/g1801_1900/s1880_check_if_word_equals_summation_of_two_words/SolutionTest.java

use leetcode_in_rust::s1880::check_if_word_equals_summation_of_two_words::Solution;

#[test]
fn test_is_sum_equal() {
    assert_eq!(
        Solution::is_sum_equal("ac".to_string(), "b".to_string(), "aaaaaaa".to_string()),
        false
    );
}

#[test]
fn test_is_sum_equal2() {
    assert_eq!(
        Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aaaa".to_string()),
        true
    );
}
