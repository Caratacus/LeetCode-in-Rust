// Tests for Problem 2605: Form Smallest Number From Two Digit Arrays
// Java reference: src/test/java/g2601_2700/s2605_form_smallest_number_from_two_digit_arrays/SolutionTest.java

use leetcode_in_rust::s2605::form_smallest_number_from_two_digit_arrays::Solution;

#[test]
fn test_min_number() {
    assert_eq!(Solution::min_number(vec![4, 1, 3], vec![5, 7]), 15);
}

#[test]
fn test_min_number2() {
    assert_eq!(Solution::min_number(vec![3, 5, 2, 6], vec![3, 1, 7]), 3);
}
