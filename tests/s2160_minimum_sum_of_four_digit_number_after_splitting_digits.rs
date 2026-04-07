// Tests for Problem 2160: Minimum Sum of Four Digit Number After Splitting Digits
// Java reference: src/test/java/g2101_2200/s2160_minimum_sum_of_four_digit_number_after_splitting_digits/SolutionTest.java

use leetcode_in_rust::s2160::minimum_sum_of_four_digit_number_after_splitting_digits::Solution;

#[test]
fn test_minimum_sum() {
    assert_eq!(Solution::minimum_sum(2932), 52);
}

#[test]
fn test_minimum_sum2() {
    assert_eq!(Solution::minimum_sum(4009), 13);
}
