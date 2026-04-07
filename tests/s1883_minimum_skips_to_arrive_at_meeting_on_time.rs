// Tests for Problem 1883: Minimum Skips to Arrive at Meeting On Time
// Java reference: src/test/java/g1801_1900/s1883_minimum_skips_to_arrive_at_meeting_on_time/SolutionTest.java

use leetcode_in_rust::s1883::minimum_skips_to_arrive_at_meeting_on_time::Solution;

#[test]
fn test_min_skips() {
    assert_eq!(Solution::min_skips(vec![1, 3, 2], 4, 2), 1);
}

#[test]
fn test_min_skips2() {
    assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 2, 10), 2);
}

#[test]
fn test_min_skips3() {
    assert_eq!(Solution::min_skips(vec![7, 3, 5, 5], 1, 10), -1);
}
