// Tests for Problem 0150: Evaluate Reverse Polish Notation
// Java reference: src/test/java/g0121_0200/s0150_evaluate_reverse_polish_notation/SolutionTest.java

use leetcode_in_rust::s0150::evaluate_reverse_polish_notation::Solution;

#[test]
fn test_eval_rpn() {
    let tokens: Vec<String> = vec!["2", "1", "+", "3", "*"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::eval_rpn(tokens), 9);
}

#[test]
fn test_eval_rpn2() {
    let tokens: Vec<String> = vec!["4", "13", "5", "/", "+"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::eval_rpn(tokens), 6);
}

#[test]
fn test_eval_rpn3() {
    let tokens: Vec<String> = vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"].iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::eval_rpn(tokens), 22);
}
