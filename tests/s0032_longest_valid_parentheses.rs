// Tests for Problem 0032: Longest Valid Parentheses
// Java reference: src/test/java/g0001_0100/s0032_longest_valid_parentheses/SolutionTest.java

use leetcode_in_rust::s0032::longest_valid_parentheses::Solution;

#[test]
fn test_longest_valid_parentheses() {
    assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
}

#[test]
fn test_longest_valid_parentheses2() {
    assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
}

#[test]
fn test_longest_valid_parentheses3() {
    assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
}
