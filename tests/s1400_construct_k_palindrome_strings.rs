// Tests for Problem 1400: Construct K Palindrome Strings
// Java reference: src/test/java/g1301_1400/s1400_construct_k_palindrome_strings/SolutionTest.java

use leetcode_in_rust::s1400::construct_k_palindrome_strings::Solution;

#[test]
fn test_can_construct() {
    assert_eq!(Solution::can_construct("annabelle".to_string(), 2), true);
}

#[test]
fn test_can_construct2() {
    assert_eq!(Solution::can_construct("leetcode".to_string(), 3), false);
}

#[test]
fn test_can_construct3() {
    assert_eq!(Solution::can_construct("true".to_string(), 4), true);
}

#[test]
fn test_can_construct4() {
    assert_eq!(Solution::can_construct("yzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzyzy".to_string(), 50), true);
}
