// Tests for Problem 0258: Add Digits
// Java reference: src/test/java/g0201_0300/s0258_add_digits/SolutionTest.java

use leetcode_in_rust::s0258::add_digits::Solution;

#[test]
fn test_add_digits() {
    assert_eq!(Solution::add_digits(38), 2);
}

#[test]
fn test_add_digits2() {
    assert_eq!(Solution::add_digits(0), 0);
}
