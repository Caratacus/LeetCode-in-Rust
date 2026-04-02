// Tests for Problem 0224: Basic Calculator
// Java reference: src/test/java/g0201_0300/s0224_basic_calculator/SolutionTest.java

use leetcode_in_rust::s0224::basic_calculator::Solution;

#[test]
fn test_calculate() {
    assert_eq!(Solution::calculate("1 + 1".to_string()), 2);
}

#[test]
fn test_calculate2() {
    assert_eq!(Solution::calculate(" 2-1 + 2 ".to_string()), 3);
}

#[test]
fn test_calculate3() {
    assert_eq!(Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()), 23);
}
