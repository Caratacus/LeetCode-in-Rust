// Tests for Problem 0415: Add Strings
// Java reference: src/test/java/g0401_0500/s0415_add_strings/SolutionTest.java

use leetcode_in_rust::s0415::add_strings::Solution;

#[test]
fn test_add_strings() {
    assert_eq!(Solution::add_strings("11".to_string(), "123".to_string()), "134");
}

#[test]
fn test_add_strings2() {
    assert_eq!(Solution::add_strings("456".to_string(), "77".to_string()), "533");
}

#[test]
fn test_add_strings3() {
    assert_eq!(Solution::add_strings("0".to_string(), "0".to_string()), "0");
}
