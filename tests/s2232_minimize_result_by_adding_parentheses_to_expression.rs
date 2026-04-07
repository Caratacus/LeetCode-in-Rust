// Tests for Problem 2232: Minimize Result by Adding Parentheses to Expression
// Java reference: src/test/java/g2201_2300/s2232_minimize_result_by_adding_parentheses_to_expression/SolutionTest.java

use leetcode_in_rust::s2232::minimize_result_by_adding_parentheses_to_expression::Solution;

#[test]
fn test_minimize_result() {
    assert_eq!(Solution::minimize_result("247+38".to_string()), "2(47+38)");
}

#[test]
fn test_minimize_result2() {
    assert_eq!(Solution::minimize_result("12+34".to_string()), "1(2+3)4");
}

#[test]
fn test_minimize_result3() {
    assert_eq!(Solution::minimize_result("999+999".to_string()), "(999+999)");
}
