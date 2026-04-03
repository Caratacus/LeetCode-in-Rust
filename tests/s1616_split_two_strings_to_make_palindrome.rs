// Tests for Problem 1616: Split Two Strings to Make Palindrome
// Java reference: src/test/java/g1601_1700/s1616_split_two_strings_to_make_palindrome/SolutionTest.java

use leetcode_in_rust::s1616::split_two_strings_to_make_palindrome::Solution;

#[test]
fn test_check_palindrome_formation() {
    assert_eq!(Solution::check_palindrome_formation("x".to_string(), "y".to_string()), true);
}

#[test]
fn test_check_palindrome_formation2() {
    assert_eq!(
        Solution::check_palindrome_formation("xbdef".to_string(), "xbdef".to_string()),
        false
    );
}

#[test]
fn test_check_palindrome_formation3() {
    assert_eq!(
        Solution::check_palindrome_formation("ulacfd".to_string(), "jizalu".to_string()),
        true
    );
}
