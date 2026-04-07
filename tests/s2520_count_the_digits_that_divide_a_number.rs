// Tests for Problem 2520: Count the Digits That Divide a Number
// Java reference: src/test/java/g2401_2500/s2520_count_the_digits_that_divide_a_number/SolutionTest.java

use leetcode_in_rust::s2520::count_the_digits_that_divide_a_number::Solution;

#[test]
fn test_count_digits() {
    assert_eq!(Solution::count_digits(7), 1);
}

#[test]
fn test_count_digits2() {
    assert_eq!(Solution::count_digits(121), 2);
}

#[test]
fn test_count_digits3() {
    assert_eq!(Solution::count_digits(1248), 4);
}
