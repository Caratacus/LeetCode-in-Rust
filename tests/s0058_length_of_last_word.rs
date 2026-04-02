// Tests for Problem 0058: Length of Last Word
// Java reference: src/test/java/g0001_0100/s0058_length_of_last_word/SolutionTest.java

use leetcode_in_rust::s0058::length_of_last_word::Solution;

#[test]
fn test_length_of_last_word() {
    assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
}

#[test]
fn test_length_of_last_word2() {
    assert_eq!(Solution::length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
}

#[test]
fn test_length_of_last_word3() {
    assert_eq!(Solution::length_of_last_word("luffy is still joyboy".to_string()), 6);
}
