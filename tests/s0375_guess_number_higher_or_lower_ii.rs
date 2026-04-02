// Tests for Problem 0375: Guess Number Higher or Lower II
// Java reference: src/test/java/g0301_0400/s0375_guess_number_higher_or_lower_ii/SolutionTest.java

use leetcode_in_rust::s0375::guess_number_higher_or_lower_ii::Solution;

#[test]
fn test_get_money_amount() {
    assert_eq!(Solution::get_money_amount(10), 16);
}

#[test]
fn test_get_money_amount2() {
    assert_eq!(Solution::get_money_amount(1), 0);
}

#[test]
fn test_get_money_amount3() {
    assert_eq!(Solution::get_money_amount(2), 1);
}
