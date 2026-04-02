// Tests for Problem 0282: Expression Add Operators
// Java reference: src/test/java/g0201_0300/s0282_expression_add_operators/SolutionTest.java

use leetcode_in_rust::s0282::expression_add_operators::Solution;

#[test]
fn test_add_operators() {
    let mut result = Solution::add_operators("123".to_string(), 6);
    result.sort();
    let mut expected = vec!["1+2+3".to_string(), "1*2*3".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_add_operators2() {
    let mut result = Solution::add_operators("232".to_string(), 8);
    result.sort();
    let mut expected = vec!["2+3*2".to_string(), "2*3+2".to_string()];
    expected.sort();
    assert_eq!(result, expected);
}

#[test]
fn test_add_operators3() {
    let result = Solution::add_operators("3456237490".to_string(), 9191);
    assert_eq!(result, Vec::<String>::new());
}
