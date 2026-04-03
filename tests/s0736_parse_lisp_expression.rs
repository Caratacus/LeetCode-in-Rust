// Tests for Problem 0736: Parse Lisp Expression
// Java reference: src/test/java/g0701_0800/s0736_parse_lisp_expression/SolutionTest.java

use leetcode_in_rust::s0736::parse_lisp_expression::Solution;

#[test]
fn test_evaluate() {
    assert_eq!(
        Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string()),
        14
    );
}

#[test]
fn test_evaluate2() {
    assert_eq!(Solution::evaluate("(let x 3 x 2 x)".to_string()), 2);
}

#[test]
fn test_evaluate3() {
    assert_eq!(
        Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string()),
        5
    );
}
