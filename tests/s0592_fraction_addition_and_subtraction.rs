// Tests for Problem 0592: Fraction Addition and Subtraction
// Java reference: src/test/java/g0501_0600/s0592_fraction_addition_and_subtraction/SolutionTest.java

use leetcode_in_rust::s0592::fraction_addition_and_subtraction::Solution;

#[test]
fn test_fraction_addition() {
    assert_eq!(Solution::fraction_addition("-1/2+1/2".to_string()), "0/1");
}

#[test]
fn test_fraction_addition2() {
    assert_eq!(Solution::fraction_addition("-1/2+1/2+1/3".to_string()), "1/3");
}

#[test]
fn test_fraction_addition3() {
    assert_eq!(Solution::fraction_addition("1/3-1/2".to_string()), "-1/6");
}
