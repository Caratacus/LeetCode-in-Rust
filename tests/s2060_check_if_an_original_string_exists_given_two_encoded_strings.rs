// Tests for Problem 2060: Check if an Original String Exists Given Two Encoded Strings
// Java reference: src/test/java/g2001_2100/s2060_check_if_an_original_string_exists_given_two_encoded_strings/SolutionTest.java

use leetcode_in_rust::s2060::check_if_an_original_string_exists_given_two_encoded_strings::Solution;

#[test]
fn test_possibly_equals() {
    assert_eq!(
        Solution::possibly_equals("internationalization".to_string(), "i18n".to_string()),
        true
    );
}

#[test]
fn test_possibly_equals2() {
    assert_eq!(Solution::possibly_equals("l123e".to_string(), "44".to_string()), true);
}

#[test]
fn test_possibly_equals3() {
    assert_eq!(Solution::possibly_equals("a5b".to_string(), "c5b".to_string()), false);
}
