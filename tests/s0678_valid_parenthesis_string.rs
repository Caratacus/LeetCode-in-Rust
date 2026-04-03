// Tests for Problem 0678: Valid Parenthesis String
// Java reference: src/test/java/g0601_0700/s0678_valid_parenthesis_string/SolutionTest.java

use leetcode_in_rust::s0678::valid_parenthesis_string::Solution;

#[test]
fn test_check_valid_string() {
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
}

#[test]
fn test_check_valid_string2() {
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
}

#[test]
fn test_check_valid_string3() {
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}
