// Tests for Problem 1568: Minimum Number of Days to Disconnect Island
// Java reference: src/test/java/g1501_1600/s1568_minimum_number_of_days_to_disconnect_island/SolutionTest.java

use leetcode_in_rust::s1568::minimum_number_of_days_to_disconnect_island::Solution;

#[test]
fn test_min_days() {
    assert_eq!(
        Solution::min_days(vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]]),
        2
    );
}

#[test]
fn test_min_days2() {
    assert_eq!(Solution::min_days(vec![vec![1, 1]]), 2);
}
