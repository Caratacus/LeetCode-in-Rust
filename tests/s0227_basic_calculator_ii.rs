// Tests for Problem 0227: Basic Calculator II
// Java reference: src/test/java/g0201_0300/s0227_basic_calculator_ii/SolutionTest.java

use leetcode_in_rust::s0227::basic_calculator_ii::Solution;

#[test]
fn test_calculate() {
    assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
}

#[test]
fn test_calculate2() {
    assert_eq!(Solution::calculate(" 3/2 ".to_string()), 1);
}

#[test]
fn test_calculate3() {
    assert_eq!(Solution::calculate(" 3+5 / 2 ".to_string()), 5);
}
