// Tests for Problem 1657: Determine if Two Strings Are Close
// Java reference: src/test/java/g1601_1700/s1657_determine_if_two_strings_are_close/SolutionTest.java

use leetcode_in_rust::s1657::determine_if_two_strings_are_close::Solution;

#[test]
fn test_close_strings() {
    assert_eq!(Solution::close_strings("abc".to_string(), "bca".to_string()), true);
}

#[test]
fn test_close_strings2() {
    assert_eq!(Solution::close_strings("a".to_string(), "aa".to_string()), false);
}

#[test]
fn test_close_strings3() {
    assert_eq!(Solution::close_strings("cabbba".to_string(), "abbccc".to_string()), true);
}

#[test]
fn test_close_strings4() {
    assert_eq!(Solution::close_strings("aa".to_string(), "aa".to_string()), true);
}

#[test]
fn test_close_strings5() {
    assert_eq!(Solution::close_strings("a".to_string(), "b".to_string()), false);
}
