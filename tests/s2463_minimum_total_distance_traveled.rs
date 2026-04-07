// Tests for Problem 2463: Minimum Total Distance Traveled
// Java reference: src/test/java/g2401_2500/s2463_minimum_total_distance_traveled/SolutionTest.java

use leetcode_in_rust::s2463::minimum_total_distance_traveled::Solution;

#[test]
fn test_minimum_total_distance() {
    assert_eq!(Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]), 4);
}

#[test]
fn test_minimum_total_distance2() {
    assert_eq!(Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]), 2);
}
