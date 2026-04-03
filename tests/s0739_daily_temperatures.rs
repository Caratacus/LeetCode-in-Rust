// Tests for Problem 0739: Daily Temperatures
// Java reference: src/test/java/g0701_0800/s0739_daily_temperatures/SolutionTest.java

use leetcode_in_rust::s0739::daily_temperatures::Solution;

#[test]
fn test_daily_temperatures() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    );
}

#[test]
fn test_daily_temperatures2() {
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    );
}

#[test]
fn test_daily_temperatures3() {
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    );
}
