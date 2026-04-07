// Tests for Problem 2187: Minimum Time to Complete Trips
// Java reference: src/test/java/g2101_2200/s2187_minimum_time_to_complete_trips/SolutionTest.java

use leetcode_in_rust::s2187::minimum_time_to_complete_trips::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(Solution::minimum_time(vec![1, 2, 3], 5), 3);
}

#[test]
fn test_minimum_time2() {
    assert_eq!(Solution::minimum_time(vec![2], 1), 2);
}
