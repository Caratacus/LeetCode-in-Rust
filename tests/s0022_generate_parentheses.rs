// Tests for Problem 0022: Generate Parentheses
// Java reference: src/test/java/g0001_0100/s0022_generate_parentheses/SolutionTest.java

use leetcode_in_rust::s0022::generate_parentheses::Solution;

#[test]
fn test_generate_parenthesis() {
    let result = Solution::generate_parenthesis(3);
    let expected = vec!["((()))", "(()())", "(())()", "()(())", "()()()"];
    assert_eq!(result.len(), expected.len());
    for p in &expected {
        assert!(result.contains(&p.to_string()));
    }
}

#[test]
fn test_generate_parenthesis2() {
    let result = Solution::generate_parenthesis(1);
    assert_eq!(result, vec!["()"]);
}
