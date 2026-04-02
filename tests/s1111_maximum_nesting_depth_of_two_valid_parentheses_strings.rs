// Tests for Problem 1111: Maximum Nesting Depth of Two Valid Parentheses Strings
// Java reference: src/test/java/g1101_1200/s1111_maximum_nesting_depth_of_two_valid_parentheses_strings/SolutionTest.java

use leetcode_in_rust::s1111::maximum_nesting_depth_of_two_valid_parentheses_strings::Solution;

#[test]
fn test_max_depth_after_split() {
    assert_eq!(Solution::max_depth_after_split("()(())()".to_string()), vec![1, 1, 1, 0, 0, 1, 1, 0]);
}

#[test]
fn test_max_depth_after_split2() {
    assert_eq!(Solution::max_depth_after_split("()()".to_string()), vec![1, 0, 1, 0]);
}
