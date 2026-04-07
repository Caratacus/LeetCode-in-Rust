// Tests for Problem 2103: Rings and Rods
// Java reference: src/test/java/g2101_2200/s2103_rings_and_rods/SolutionTest.java

use leetcode_in_rust::s2103::rings_and_rods::Solution;

#[test]
fn test_count_points() {
    assert_eq!(Solution::count_points("B0B6G0R6R0R6G9".to_string()), 1);
}

#[test]
fn test_count_points2() {
    assert_eq!(Solution::count_points("B0R0G0R9R0B0G0".to_string()), 1);
}

#[test]
fn test_count_points3() {
    assert_eq!(Solution::count_points("G4".to_string()), 0);
}
