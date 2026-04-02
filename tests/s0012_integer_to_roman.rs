// Tests for Problem 0012: Integer to Roman
// Java reference: src/test/java/g0001_0100/s0012_integer_to_roman/SolutionTest.java

use leetcode_in_rust::s0012::integer_to_roman::Solution;

#[test]
fn test_int_to_roman() {
    assert_eq!(Solution::int_to_roman(3), "III");
}

#[test]
fn test_int_to_roman2() {
    assert_eq!(Solution::int_to_roman(4), "IV");
}

#[test]
fn test_int_to_roman3() {
    assert_eq!(Solution::int_to_roman(9), "IX");
}

#[test]
fn test_int_to_roman4() {
    assert_eq!(Solution::int_to_roman(58), "LVIII");
}

#[test]
fn test_int_to_roman5() {
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
