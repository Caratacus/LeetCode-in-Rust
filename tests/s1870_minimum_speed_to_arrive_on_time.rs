// Tests for Problem 1870: Minimum Speed to Arrive on Time
// Java reference: src/test/java/g1801_1900/s1870_minimum_speed_to_arrive_on_time/SolutionTest.java

use leetcode_in_rust::s1870::minimum_speed_to_arrive_on_time::Solution;

#[test]
fn test_min_speed_on_time() {
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 6.0), 1);
}

#[test]
fn test_min_speed_on_time2() {
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 2.7), 3);
}

#[test]
fn test_min_speed_on_time3() {
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 1.9), -1);
}
