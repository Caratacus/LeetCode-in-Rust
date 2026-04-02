// Tests for Problem 0151: Reverse Words in a String
// Java reference: src/test/java/g0121_0200/s0151_reverse_words_in_a_string/SolutionTest.java

use leetcode_in_rust::s0151::reverse_words_in_a_string::Solution;

#[test]
fn test_reverse_words() {
    assert_eq!(Solution::reverse_words(String::from("the sky is blue")), String::from("blue is sky the"));
}

#[test]
fn test_reverse_words2() {
    assert_eq!(Solution::reverse_words(String::from("  hello world  ")), String::from("world hello"));
}

#[test]
fn test_reverse_words3() {
    assert_eq!(Solution::reverse_words(String::from("a good   example")), String::from("example good a"));
}
