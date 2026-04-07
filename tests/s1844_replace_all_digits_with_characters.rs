// Tests for Problem 1844: Replace All Digits with Characters
// Java reference: src/test/java/g1801_1900/s1844_replace_all_digits_with_characters/SolutionTest.java

use leetcode_in_rust::s1844::replace_all_digits_with_characters::Solution;

#[test]
fn test_replace_digits() {
    assert_eq!(
        Solution::replace_digits("a1c1e1".to_string()),
        "abcdef".to_string()
    );
}

#[test]
fn test_replace_digits2() {
    assert_eq!(
        Solution::replace_digits("a1b2c3d4e".to_string()),
        "abbdcfdhe".to_string()
    );
}
