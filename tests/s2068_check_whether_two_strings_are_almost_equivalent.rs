// Tests for Problem 2068: Check Whether Two Strings are Almost Equivalent
// Java reference: src/test/java/g2001_2100/s2068_check_whether_two_strings_are_almost_equivalent/SolutionTest.java

use leetcode_in_rust::s2068::check_whether_two_strings_are_almost_equivalent::Solution;

#[test]
fn test_check_almost_equivalent() {
    assert_eq!(
        Solution::check_almost_equivalent("aaaa".to_string(), "bccb".to_string()),
        false
    );
}

#[test]
fn test_check_almost_equivalent2() {
    assert_eq!(
        Solution::check_almost_equivalent("abcdeef".to_string(), "abaaacc".to_string()),
        true
    );
}

#[test]
fn test_check_almost_equivalent3() {
    assert_eq!(
        Solution::check_almost_equivalent("cccddabba".to_string(), "babababab".to_string()),
        true
    );
}
