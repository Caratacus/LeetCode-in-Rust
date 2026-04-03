// Tests for Problem 0738: Monotone Increasing Digits
// Java reference: src/test/java/g0701_0800/s0738_monotone_increasing_digits/SolutionTest.java

use leetcode_in_rust::s0738::monotone_increasing_digits::Solution;

#[test]
fn test_monotone_increasing_digits() {
    assert_eq!(Solution::monotone_increasing_digits(10), 9);
}

#[test]
fn test_monotone_increasing_digits2() {
    assert_eq!(Solution::monotone_increasing_digits(1234), 1234);
}

#[test]
fn test_monotone_increasing_digits3() {
    assert_eq!(Solution::monotone_increasing_digits(332), 299);
}
