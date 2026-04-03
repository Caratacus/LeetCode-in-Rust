// Tests for Problem 1523: Count Odd Numbers in an Interval Range
// Java reference: src/test/java/g1501_1600/s1523_count_odd_numbers_in_an_interval_range/SolutionTest.java

use leetcode_in_rust::s1523::count_odd_numbers_in_an_interval_range::Solution;

#[test]
fn test_count_odds() {
    assert_eq!(Solution::count_odds(3, 7), 3);
}

#[test]
fn test_count_odds2() {
    assert_eq!(Solution::count_odds(8, 10), 1);
}
