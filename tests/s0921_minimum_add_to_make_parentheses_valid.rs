// Tests for Problem 0921: Minimum Add to Make Parentheses Valid
// Java reference: src/test/java/g0901_1000/s0921_minimum_add_to_make_parentheses_valid/SolutionTest.java

use leetcode_in_rust::s0921::minimum_add_to_make_parentheses_valid::Solution;

#[test]
fn test_min_add_to_make_valid() {
    assert_eq!(Solution::min_add_to_make_valid("())".to_string()), 1);
}

#[test]
fn test_min_add_to_make_valid2() {
    assert_eq!(Solution::min_add_to_make_valid("(((".to_string()), 3);
}
