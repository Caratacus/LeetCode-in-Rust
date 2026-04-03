// Tests for Problem 0856: Score of Parentheses
// Java reference: src/test/java/g0801_0900/s0856_score_of_parentheses/SolutionTest.java

use leetcode_in_rust::s0856::score_of_parentheses::Solution;

#[test]
fn test_score_of_parentheses() {
    assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
}

#[test]
fn test_score_of_parentheses2() {
    assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
}

#[test]
fn test_score_of_parentheses3() {
    assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
}
