// Tests for Problem 1401: Circle and Rectangle Overlapping
// Java reference: src/test/java/g1301_1400/s1401_circle_and_rectangle_overlapping/SolutionTest.java

use leetcode_in_rust::s1401::circle_and_rectangle_overlapping::Solution;

#[test]
fn test_check_overlap() {
    // radius=1, x_center=0, y_center=0, x1=-1, y1=-1, x2=1, y2=1
    assert_eq!(Solution::check_overlap(1, 0, 0, -1, -1, 1, 1), true);
}

#[test]
fn test_check_overlap2() {
    // radius=1, x_center=1, y_center=1, x1=-3, y1=-3, x2=2, y2=2
    assert_eq!(Solution::check_overlap(1, 1, 1, -3, -3, 2, 2), true);
}

#[test]
fn test_check_overlap3() {
    // radius=1, x_center=0, y_center=0, x1=-1, y1=0, x2=0, y2=1
    assert_eq!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1), true);
}
