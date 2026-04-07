// Tests for Problem 2243: Calculate Digit Sum of a String
// Java reference: src/test/java/g2201_2300/s2243_calculate_digit_sum_of_a_string/SolutionTest.java

use leetcode_in_rust::s2243::calculate_digit_sum_of_a_string::Solution;

#[test]
fn test_digit_sum() {
    assert_eq!(Solution::digit_sum("11111222223".to_string(), 3), "135");
}

#[test]
fn test_digit_sum2() {
    assert_eq!(Solution::digit_sum("00000000".to_string(), 3), "000");
}
