// Tests for Problem 0172: Factorial Trailing Zeroes
// Java reference: src/test/java/g0121_0200/s0172_factorial_trailing_zeroes/SolutionTest.java

use leetcode_in_rust::s0172::factorial_trailing_zeroes::Solution;

#[test]
fn test_trailing_zeroes() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
}

#[test]
fn test_trailing_zeroes2() {
    assert_eq!(Solution::trailing_zeroes(5), 1);
}

#[test]
fn test_trailing_zeroes3() {
    assert_eq!(Solution::trailing_zeroes(0), 0);
}
