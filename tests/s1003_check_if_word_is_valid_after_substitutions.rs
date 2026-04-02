// Tests for Problem 1003: Check If Word Is Valid After Substitutions
// Java reference: src/test/java/g1001_1100/s1003_check_if_word_is_valid_after_substitutions/SolutionTest.java

use leetcode_in_rust::s1003::check_if_word_is_valid_after_substitutions::Solution;

#[test]
fn test_is_valid() {
    assert_eq!(Solution::is_valid("aabcbc".to_string()), true);
}

#[test]
fn test_is_valid2() {
    assert_eq!(Solution::is_valid("abcabcababcc".to_string()), true);
}

#[test]
fn test_is_valid3() {
    assert_eq!(Solution::is_valid("abccba".to_string()), false);
}
