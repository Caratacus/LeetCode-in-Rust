// Tests for Problem 2486: Append Characters to String to Make Subsequence
// Java reference: src/test/java/g2401_2500/s2486_append_characters_to_string_to_make_subsequence/SolutionTest.java

use leetcode_in_rust::s2486::append_characters_to_string_to_make_subsequence::Solution;

#[test]
fn test_append_characters() {
    assert_eq!(Solution::append_characters("coaching".to_string(), "coding".to_string()), 4);
}

#[test]
fn test_append_characters2() {
    assert_eq!(Solution::append_characters("abcde".to_string(), "a".to_string()), 0);
}

#[test]
fn test_append_characters3() {
    assert_eq!(Solution::append_characters("z".to_string(), "abcde".to_string()), 5);
}
