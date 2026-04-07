// Tests for Problem 2310: Sum of Numbers With Units Digit K
// Java reference: src/test/java/g2301_2400/s2310_sum_of_numbers_with_units_digit_k/SolutionTest.java

use leetcode_in_rust::s2310::sum_of_numbers_with_units_digit_k::Solution;

#[test]
fn test_minimum_numbers() {
    assert_eq!(Solution::minimum_numbers(58, 9), 2);
}

#[test]
fn test_minimum_numbers2() {
    assert_eq!(Solution::minimum_numbers(37, 2), -1);
}

#[test]
fn test_minimum_numbers3() {
    assert_eq!(Solution::minimum_numbers(0, 7), 0);
}

#[test]
fn test_minimum_numbers4() {
    assert_eq!(Solution::minimum_numbers(2, 8), -1);
}
