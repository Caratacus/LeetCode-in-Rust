// Tests for Problem 0007: Reverse Integer
// Java reference: src/test/java/g0001_0100/s0007_reverse_integer/SolutionTest.java

use leetcode_in_rust::s0007::reverse_integer::Solution;

#[test]
fn test_reverse() {
    assert_eq!(Solution::reverse(123), 321);
}

#[test]
fn test_reverse2() {
    assert_eq!(Solution::reverse(-123), -321);
}

#[test]
fn test_reverse3() {
    assert_eq!(Solution::reverse(120), 21);
}
