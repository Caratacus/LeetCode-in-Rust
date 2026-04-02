// Tests for Problem 0301: Remove Invalid Parentheses
// Java reference: src/test/java/g0301_0400/s0301_remove_invalid_parentheses/SolutionTest.java

use leetcode_in_rust::s0301::remove_invalid_parentheses::Solution;

#[test]
fn test_remove_invalid_parentheses() {
    let mut result = Solution::remove_invalid_parentheses("()())()".to_string());
    result.sort();
    let mut expected = vec!["(())()".to_string(), "()()()".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_remove_invalid_parentheses2() {
    let mut result = Solution::remove_invalid_parentheses("(a)())()".to_string());
    result.sort();
    let mut expected = vec!["(a())()".to_string(), "(a)()()".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_remove_invalid_parentheses3() {
    let result = Solution::remove_invalid_parentheses(")(".to_string());
    assert_eq!(result, vec![""]);
}
