// Tests for Problem 0233: Number of Digit One
// Java reference: src/test/java/g0201_0300/s0233_number_of_digit_one/SolutionTest.java

use leetcode_in_rust::s0233::number_of_digit_one::Solution;

#[test]
fn test_count_digit_one() {
    assert_eq!(Solution::count_digit_one(13), 6);
}

#[test]
fn test_count_digit_one2() {
    assert_eq!(Solution::count_digit_one(0), 0);
}
