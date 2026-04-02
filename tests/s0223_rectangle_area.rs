// Tests for Problem 0223: Rectangle Area
// Java reference: src/test/java/g0201_0300/s0223_rectangle_area/SolutionTest.java

use leetcode_in_rust::s0223::rectangle_area::Solution;

#[test]
fn test_compute_area() {
    assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
}

#[test]
fn test_compute_area2() {
    assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
}
