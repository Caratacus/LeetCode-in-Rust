// Tests for Problem 2739: Total Distance Traveled
// Java reference: src/test/java/g2701_2800/s2739_total_distance_traveled/SolutionTest.java

use leetcode_in_rust::s2739::total_distance_traveled::Solution;

#[test]
fn test_distance_traveled() {
    assert_eq!(Solution::distance_traveled(5, 10), 60);
}

#[test]
fn test_distance_traveled2() {
    assert_eq!(Solution::distance_traveled(1, 2), 10);
}
