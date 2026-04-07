// Tests for Problem 2283: Check if Number Has Equal Digit Count and Digit Value
// Java reference: src/test/java/g2201_2300/s2283_check_if_number_has_equal_digit_count_and_digit_value/SolutionTest.java

use leetcode_in_rust::s2283::check_if_number_has_equal_digit_count_and_digit_value::Solution;

#[test]
fn test_digit_count() {
    assert_eq!(Solution::digit_count(String::from("1210")), true);
}

#[test]
fn test_digit_count2() {
    assert_eq!(Solution::digit_count(String::from("030")), false);
}
