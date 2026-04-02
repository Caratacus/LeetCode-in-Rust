// Tests for Problem 1184: Distance Between Bus Stops
// Java reference: src/test/java/g1101_1200/s1184_distance_between_bus_stops/SolutionTest.java

use leetcode_in_rust::s1184::distance_between_bus_stops::Solution;

#[test]
fn test_distance_between_bus_stops() {
    assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1), 1);
}

#[test]
fn test_distance_between_bus_stops2() {
    assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2), 3);
}

#[test]
fn test_distance_between_bus_stops3() {
    assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3), 4);
}

#[test]
fn test_distance_between_bus_stops4() {
    assert_eq!(Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 3, 1), 5);
}

#[test]
fn test_distance_between_bus_stops5() {
    assert_eq!(Solution::distance_between_bus_stops(vec![7, 1, 2, 3], 1, 3), 3);
}

#[test]
fn test_distance_between_bus_stops6() {
    assert_eq!(Solution::distance_between_bus_stops(vec![2, 2, 2, 2], 1, 3), 4);
}

#[test]
fn test_distance_between_bus_stops7() {
    assert_eq!(Solution::distance_between_bus_stops(vec![5], 0, 0), 0);
}

#[test]
fn test_distance_between_bus_stops8() {
    assert_eq!(Solution::distance_between_bus_stops(vec![3, 8], 1, 0), 3);
}

#[test]
fn test_distance_between_bus_stops9() {
    assert_eq!(Solution::distance_between_bus_stops(vec![1, 4, 6, 3], 2, 1), 4);
}
