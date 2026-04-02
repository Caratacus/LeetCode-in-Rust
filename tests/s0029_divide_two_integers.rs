// Tests for Problem 0029: Divide Two Integers
// Java reference: src/test/java/g0001_0100/s0029_divide_two_integers/SolutionTest.java

use leetcode_in_rust::s0029::divide_two_integers::Solution;

#[test]
fn test_divide() {
    assert_eq!(Solution::divide(10, 3), 3);
}

#[test]
fn test_divide2() {
    assert_eq!(Solution::divide(7, -3), -2);
}

#[test]
fn test_divide3() {
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
}
