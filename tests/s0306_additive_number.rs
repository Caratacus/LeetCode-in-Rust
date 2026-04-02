// Tests for Problem 0306: Additive Number
// Java reference: src/test/java/g0301_0400/s0306_additive_number/SolutionTest.java

use leetcode_in_rust::s0306::additive_number::Solution;

#[test]
fn test_is_additive_number() {
    assert_eq!(Solution::is_additive_number("112358".to_string()), true);
}

#[test]
fn test_is_additive_number2() {
    assert_eq!(Solution::is_additive_number("199100199".to_string()), true);
}

#[test]
fn test_is_additive_number3() {
    assert_eq!(Solution::is_additive_number("0235813".to_string()), false);
}

#[test]
fn test_is_additive_number4() {
    assert_eq!(Solution::is_additive_number("000".to_string()), true);
}

#[test]
fn test_is_additive_number5() {
    assert_eq!(Solution::is_additive_number("011235".to_string()), true);
}
