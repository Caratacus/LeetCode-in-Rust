// Tests for Problem 2943: Maximize Area of Square Hole in Grid
// Java reference: src/test/java/g2901_3000/s2943_maximize_area_of_square_hole_in_grid/SolutionTest.java

use leetcode_in_rust::s2943::maximize_area_of_square_hole_in_grid::Solution;

#[test]
fn test_maximize_square_hole_area() {
    assert_eq!(Solution::maximize_square_hole_area(2, 1, vec![2, 3], vec![2]), 4);
}

#[test]
fn test_maximize_square_hole_area2() {
    assert_eq!(Solution::maximize_square_hole_area(1, 1, vec![2], vec![2]), 4);
}

#[test]
fn test_maximize_square_hole_area3() {
    assert_eq!(Solution::maximize_square_hole_area(2, 3, vec![2, 3], vec![2, 3, 4]), 9);
}
