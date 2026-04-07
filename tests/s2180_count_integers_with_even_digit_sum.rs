// Tests for Problem 2180: Count Integers With Even Digit Sum
// Java reference: src/test/java/g2101_2200/s2180_count_integers_with_even_digit_sum/SolutionTest.java

use leetcode_in_rust::s2180::count_integers_with_even_digit_sum::Solution;

#[test]
fn test_count_even() {
    assert_eq!(Solution::count_even(4), 2);
}

#[test]
fn test_count_even2() {
    assert_eq!(Solution::count_even(30), 14);
}

#[test]
fn test_count_even3() {
    assert_eq!(Solution::count_even(11), 5);
}
