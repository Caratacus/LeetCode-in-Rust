// Tests for Problem 2651: Calculate Delayed Arrival Time
// Java reference: src/test/java/g2601_2700/s2651_calculate_delayed_arrival_time/SolutionTest.java

use leetcode_in_rust::s2651::calculate_delayed_arrival_time::Solution;

#[test]
fn test_find_delayed_arrival_time() {
    assert_eq!(Solution::find_delayed_arrival_time(15, 5), 20);
}

#[test]
fn test_find_delayed_arrival_time2() {
    assert_eq!(Solution::find_delayed_arrival_time(13, 11), 0);
}
