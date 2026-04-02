// Tests for Problem 0290: Word Pattern
// Java reference: src/test/java/g0201_0300/s0290_word_pattern/SolutionTest.java

use leetcode_in_rust::s0290::word_pattern::Solution;

#[test]
fn test_word_pattern() {
    assert_eq!(Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()), true);
}

#[test]
fn test_word_pattern2() {
    assert_eq!(Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()), false);
}

#[test]
fn test_word_pattern3() {
    assert_eq!(Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()), false);
}

#[test]
fn test_word_pattern4() {
    assert_eq!(Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()), false);
}
