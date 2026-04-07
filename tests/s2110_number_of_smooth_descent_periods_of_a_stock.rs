// Tests for Problem 2110: Number of Smooth Descent Periods of a Stock
// Java reference: src/test/java/g2101_2200/s2110_number_of_smooth_descent_periods_of_a_stock/SolutionTest.java

use leetcode_in_rust::s2110::number_of_smooth_descent_periods_of_a_stock::Solution;

#[test]
fn test_get_descent_periods() {
    assert_eq!(Solution::get_descent_periods(vec![3, 2, 1, 4]), 7);
}

#[test]
fn test_get_descent_periods2() {
    assert_eq!(Solution::get_descent_periods(vec![8, 6, 7, 7]), 4);
}

#[test]
fn test_get_descent_periods3() {
    assert_eq!(Solution::get_descent_periods(vec![1]), 1);
}
