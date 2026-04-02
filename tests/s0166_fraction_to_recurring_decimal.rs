// Tests for Problem 0166: Fraction to Recurring Decimal
// Java reference: src/test/java/g0121_0200/s0166_fraction_to_recurring_decimal/SolutionTest.java

use leetcode_in_rust::s0166::fraction_to_recurring_decimal::Solution;

#[test]
fn test_fraction_to_decimal() {
    assert_eq!(Solution::fraction_to_decimal(1, 2), String::from("0.5"));
}

#[test]
fn test_fraction_to_decimal2() {
    assert_eq!(Solution::fraction_to_decimal(2, 1), String::from("2"));
}

#[test]
fn test_fraction_to_decimal3() {
    assert_eq!(Solution::fraction_to_decimal(4, 333), String::from("0.(012)"));
}
