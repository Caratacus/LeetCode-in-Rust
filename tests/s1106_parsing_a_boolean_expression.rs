// Tests for Problem 1106: Parsing a Boolean Expression
// Java reference: src/test/java/g1101_1200/s1106_parsing_a_boolean_expression/SolutionTest.java

use leetcode_in_rust::s1106::parsing_a_boolean_expression::Solution;

#[test]
fn test_parse_bool_expr() {
    assert_eq!(Solution::parse_bool_expr("!(f)".to_string()), false);
}

#[test]
fn test_parse_bool_expr2() {
    assert_eq!(Solution::parse_bool_expr("|(f,t,f)".to_string()), true);
}

#[test]
fn test_parse_bool_expr3() {
    assert_eq!(Solution::parse_bool_expr("&(t,f)".to_string()), false);
}

#[test]
fn test_parse_bool_expr4() {
    assert_eq!(Solution::parse_bool_expr("!(|(f,f,t))".to_string()), true);
}
