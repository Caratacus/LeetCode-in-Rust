// Tests for Problem 1021: Remove Outermost Parentheses
// Java reference: src/test/java/g1001_1100/s1021_remove_outermost_parentheses/SolutionTest.java

use leetcode_in_rust::s1021::remove_outermost_parentheses::Solution;

#[test]
fn test_remove_outer_parentheses() {
    assert_eq!(Solution::remove_outer_parentheses("(()())(())".to_string()), "()()()");
}

#[test]
fn test_remove_outer_parentheses2() {
    assert_eq!(
        Solution::remove_outer_parentheses("(()())(())(()(()))".to_string()),
        "()()()()(())"
    );
}

#[test]
fn test_remove_outer_parentheses3() {
    assert_eq!(Solution::remove_outer_parentheses("()()".to_string()), "");
}
