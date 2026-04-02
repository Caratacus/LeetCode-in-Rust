// Tests for Problem 0020: Valid Parentheses
// Java reference: src/test/java/g0001_0100/s0020_valid_parentheses/SolutionTest.java

use leetcode_in_rust::s0020::valid_parentheses::Solution;

#[test]
fn test_is_valid() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
}

#[test]
fn test_is_valid2() {
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
}

#[test]
fn test_is_valid3() {
    assert_eq!(Solution::is_valid("(]".to_string()), false);
}

#[test]
fn test_is_valid4() {
    assert_eq!(Solution::is_valid("([)]".to_string()), false);
}

#[test]
fn test_is_valid5() {
    assert_eq!(Solution::is_valid("{[]}".to_string()), true);
}
