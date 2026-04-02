// Tests for Problem 0611: Valid Triangle Number
// Java reference: src/test/java/g0601_0700/s0611_valid_triangle_number/SolutionTest.java

use leetcode_in_rust::s0611::valid_triangle_number::Solution;

#[test]
fn test_triangle_number() {
    assert_eq!(Solution::triangle_number(vec![2, 2, 3, 4]), 3);
}

#[test]
fn test_triangle_number2() {
    assert_eq!(Solution::triangle_number(vec![4, 2, 3, 4]), 4);
}
