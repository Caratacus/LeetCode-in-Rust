// Tests for Problem 0871: Minimum Number of Refueling Stops
// Java reference: src/test/java/g0801_0900/s0871_minimum_number_of_refueling_stops/SolutionTest.java

use leetcode_in_rust::s0871::minimum_number_of_refueling_stops::Solution;

#[test]
fn test_min_refuel_stops() {
    assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
}

#[test]
fn test_min_refuel_stops2() {
    assert_eq!(Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
}

#[test]
fn test_min_refuel_stops3() {
    assert_eq!(
        Solution::min_refuel_stops(100, 10, vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]),
        2
    );
}
