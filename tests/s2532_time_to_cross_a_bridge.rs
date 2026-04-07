// Tests for Problem 2532: Time to Cross a Bridge
// Java reference: src/test/java/g2501_2600/s2532_time_to_cross_a_bridge/SolutionTest.java

use leetcode_in_rust::s2532::time_to_cross_a_bridge::Solution;

#[test]
fn test_find_crossing_time() {
    assert_eq!(
        Solution::find_crossing_time(1, 3, vec![vec![1, 1, 2, 1], vec![1, 1, 3, 1], vec![1, 1, 4, 1]]),
        6
    );
}

#[test]
fn test_find_crossing_time2() {
    assert_eq!(
        Solution::find_crossing_time(3, 2, vec![vec![1, 9, 1, 8], vec![10, 10, 10, 10]]),
        50
    );
}
