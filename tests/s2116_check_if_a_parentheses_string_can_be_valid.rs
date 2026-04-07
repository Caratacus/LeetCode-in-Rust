// Tests for Problem 2116: Check if a Parentheses String Can Be Valid
// Java reference: src/test/java/g2101_2200/s2116_check_if_a_parentheses_string_can_be_valid/SolutionTest.java

use leetcode_in_rust::s2116::check_if_a_parentheses_string_can_be_valid::Solution;

#[test]
fn test_can_be_valid() {
    assert_eq!(Solution::can_be_valid("))()))".to_string(), "010100".to_string()), true);
}

#[test]
fn test_can_be_valid2() {
    assert_eq!(Solution::can_be_valid("()()".to_string(), "0000".to_string()), true);
}

#[test]
fn test_can_be_valid3() {
    assert_eq!(Solution::can_be_valid(")".to_string(), "0".to_string()), false);
}
