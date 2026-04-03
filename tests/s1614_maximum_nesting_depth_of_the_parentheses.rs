// Tests for Problem 1614: Maximum Nesting Depth of the Parentheses
// Java reference: src/test/java/g1601_1700/s1614_maximum_nesting_depth_of_the_parentheses/SolutionTest.java

use leetcode_in_rust::s1614::maximum_nesting_depth_of_the_parentheses::Solution;

#[test]
fn test_max_depth() {
    assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
}

#[test]
fn test_max_depth2() {
    assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
}
