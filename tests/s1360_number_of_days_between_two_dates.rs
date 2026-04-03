// Tests for Problem 1360: Number of Days Between Two Dates
// Java reference: src/test/java/g1301_1400/s1360_number_of_days_between_two_dates/SolutionTest.java

use leetcode_in_rust::s1360::number_of_days_between_two_dates::Solution;

#[test]
fn test_days_between_dates() {
    assert_eq!(Solution::days_between_dates("2019-06-29".to_string(), "2019-06-30".to_string()), 1);
}

#[test]
fn test_days_between_dates2() {
    assert_eq!(Solution::days_between_dates("2020-01-15".to_string(), "2019-12-31".to_string()), 15);
}
