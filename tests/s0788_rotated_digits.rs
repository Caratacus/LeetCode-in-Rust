// Tests for Problem 0788: Rotated Digits
// Java reference: src/test/java/g0701_0800/s0788_rotated_digits/SolutionTest.java

use leetcode_in_rust::s0788::rotated_digits::Solution;

#[test]
fn test_rotated_digits() {
    assert_eq!(Solution::rotated_digits(10), 4);
}

#[test]
fn test_rotated_digits2() {
    assert_eq!(Solution::rotated_digits(1), 0);
}

#[test]
fn test_rotated_digits3() {
    assert_eq!(Solution::rotated_digits(2), 1);
}

#[test]
fn test_rotated_digits4() {
    assert_eq!(Solution::rotated_digits(857), 247);
}

#[test]
fn test_rotated_digits5() {
    assert_eq!(Solution::rotated_digits(15), 6);
}
